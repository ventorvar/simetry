use crate::iracing::constants::IRSDK_VER;
use crate::iracing::header::{VarBuf, VarHeaderRaw};
use crate::iracing::session_info::{parse_session_info, SessionInfo};
use crate::iracing::{Header, SimState, VarHeader, VarHeaders};
use crate::{windows_util, Moment, Simetry};
use anyhow::{bail, Result};
use std::collections::HashMap;
use std::slice::from_raw_parts;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::task::spawn_blocking;
use windows::core::PCSTR;
use windows::Win32::System::Threading::{
    OpenEventA, WaitForSingleObject, INFINITE, SYNCHRONIZATION_SYNCHRONIZE,
};

static DATAVALIDEVENTNAME: &[u8] = b"Local\\IRSDKDataValidEvent\0";
static MEMMAPFILENAME: &[u8] = b"Local\\IRSDKMemMapFileName\0";

const STATUS_CONNECTED_FLAG: i32 = 1;

const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

#[cfg(debug_assertions)]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        if std::env::var("IRACING_DEBUG").is_ok() {
            println!($($arg)*);
        }
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug_print {
    ($($arg:tt)*) => {};
}

pub struct Client {
    vars_at_buf_len: i32,
    vars: Arc<VarHeaders>,
    session_info_cache: SessionInfoCache,
    last_tick_count: i32,
    last_valid_time: Option<SystemTime>,

    shared_memory: SharedMemory,
    data_valid_event: DataValidEvent,
}

impl Client {
    pub async fn connect(retry_delay: Duration) -> Self {
        loop {
            if let Ok(v) = Self::try_connect().await {
                return v;
            }
            tokio::time::sleep(retry_delay).await;
        }
    }

    pub async fn try_connect() -> Result<Self> {
        let shared_memory = SharedMemory::connect();
        let data_valid_event = DataValidEvent::connect();

        let shared_memory = shared_memory.await;
        let data_valid_event = data_valid_event.await;

        while !shared_memory.is_header_connected() {
            data_valid_event
                .wait(Some(Duration::from_millis(250)))
                .await;
        }

        let sdk_version = shared_memory.header().ver;
        if sdk_version != IRSDK_VER {
            bail!("iRacing SDK version mismatch: expected {IRSDK_VER}, received {sdk_version}");
        }

        // Initialize last_tick_count to the current tick count minus 1
        let header = shared_memory.header();
        let mut last_tick_count = -1;
        for idx in 0..(header.num_buf as usize) {
            let tick = header.var_buf[idx].tick_count;
            if tick > last_tick_count {
                last_tick_count = tick;
            }
        }
        last_tick_count = last_tick_count.wrapping_sub(1); // Start from the previous tick

        Ok(Client {
            vars_at_buf_len: -1,
            vars: Arc::new(HashMap::new()),
            session_info_cache: SessionInfoCache::default(),
            last_tick_count,
            last_valid_time: None,
            shared_memory,
            data_valid_event,
        })
    }

    pub async fn next_sim_state(&mut self) -> Option<SimState> {
        let start = std::time::Instant::now();
        
        if !self.is_connected() {
            debug_print!("Client not connected");
            return None;
        }

        // Try to get new data immediately
        if let Some(sim_state) = self.get_new_sim_state() {
            let get_time = start.elapsed();
            debug_print!("Client timing: get_new_sim_state={:?}, total={:?}", get_time, get_time);
            return Some(sim_state);
        }

        debug_print!("No immediate data, waiting for data valid event...");
        // If no new data, wait briefly for the data valid event
        self.data_valid_event
            .wait(Some(Duration::from_micros(100)))
            .await;

        // Try one more time after the wait
        if let Some(sim_state) = self.get_new_sim_state() {
            let get_time = start.elapsed();
            debug_print!("Client timing: get_new_sim_state={:?}, total={:?}", get_time, get_time);
            return Some(sim_state);
        }

        debug_print!("No data available after wait");
        None
    }

    fn get_new_sim_state(&mut self) -> Option<SimState> {
        let header = self.shared_memory.header();

        if self.vars_at_buf_len != header.buf_len {
            debug_print!("Updating var headers, old len={}, new len={}", self.vars_at_buf_len, header.buf_len);
            self.vars = Arc::new(self.shared_memory.get_var_headers());
        }

        if header.status & STATUS_CONNECTED_FLAG == 0 {
            debug_print!("Header not connected, status={}", header.status);
            self.last_tick_count = i32::MAX;
            return None;
        }

        // Find all buffers with new data
        let mut new_buffers = Vec::new();
        for idx in 0..(header.num_buf as usize) {
            let tick = header.var_buf[idx].tick_count;
            if tick > self.last_tick_count {
                new_buffers.push((idx, tick));
            }
        }

        // Sort by tick count to process in order
        new_buffers.sort_by_key(|&(_, tick)| tick);

        // Process the latest buffer
        if let Some(&(latest_idx, latest_tick)) = new_buffers.last() {
            let buffer = &header.var_buf[latest_idx];
            debug_print!("Found new data: current={}, last={}, diff={}, new_buffers={}", 
                buffer.tick_count, self.last_tick_count, 
                buffer.tick_count.wrapping_sub(self.last_tick_count),
                new_buffers.len());

            // Read the data
            let data = self.shared_memory.data(header, buffer);
            self.last_tick_count = latest_tick;
            self.last_valid_time = Some(SystemTime::now());

            if self.is_connected() {
                let session_info = self.session_info_cache.get(&self.shared_memory).ok()?;
                return Some(SimState::new(
                    Arc::new(header.clone()),
                    Arc::clone(&self.vars),
                    data.to_vec(),
                    session_info,
                    latest_tick,
                ));
            }
        } else {
            debug_print!("No new buffers found, last_tick={}", self.last_tick_count);
        }

        None
    }

    pub fn is_connected(&self) -> bool {
        if !self.shared_memory.is_header_connected() {
            return false;
        }
        let last_valid_time = match self.last_valid_time {
            Some(v) => v,
            None => {
                return true;
            }
        };
        let elapsed = match SystemTime::now().duration_since(last_valid_time) {
            Ok(v) => v,
            Err(_) => return false,
        };
        elapsed < CLIENT_TIMEOUT
    }
}

#[async_trait::async_trait]
impl Simetry for Client {
    fn name(&self) -> &str {
        "iRacing"
    }

    async fn next_moment(&mut self) -> Option<Box<dyn Moment + Send + Sync + 'static>> {
        Some(Box::new(self.next_sim_state().await?))
    }
}

#[derive(Default)]
struct SessionInfoCache {
    content: Option<(i32, Arc<SessionInfo>)>,
}

impl SessionInfoCache {
    fn get(&mut self, shared_memory: &SharedMemory) -> Result<Arc<SessionInfo>> {
        let new_id = shared_memory.header().session_info_update;
        if let Some((old_id, data)) = &self.content {
            if new_id == *old_id {
                return Ok(Arc::clone(data));
            }
        }
        let session_info = Arc::new(parse_session_info(shared_memory.raw_session_info())?);
        self.content = Some((new_id, Arc::clone(&session_info)));
        Ok(session_info)
    }
}

struct SharedMemory(windows_util::SharedMemory);

impl SharedMemory {
    async fn connect() -> Self {
        Self(windows_util::SharedMemory::connect(MEMMAPFILENAME, Duration::from_millis(1)).await)
    }

    fn header(&self) -> &Header {
        unsafe { &*(self.0.get() as *const Header) }
    }

    fn is_header_connected(&self) -> bool {
        (self.header().status & STATUS_CONNECTED_FLAG) != 0
    }

    fn raw_var_headers(&self) -> &[VarHeaderRaw] {
        let header = self.header();
        unsafe {
            from_raw_parts(
                (self.0.get() as *const u8).offset(header.var_header_offset as isize)
                    as *const VarHeaderRaw,
                header.num_vars as usize,
            )
        }
    }

    fn get_var_headers(&self) -> VarHeaders {
        self.raw_var_headers()
            .iter()
            .filter_map(|var_header_raw| {
                let var_header = VarHeader::from_raw(var_header_raw).ok()?;
                Some((var_header.name.clone(), var_header))
            })
            .collect()
    }

    fn data(&self, header: &Header, buffer: &VarBuf) -> &[u8] {
        unsafe {
            from_raw_parts(
                (self.0.get() as *const u8).offset(buffer.buf_offset as isize),
                header.buf_len as usize,
            )
        }
    }

    fn raw_session_info(&self) -> &[u8] {
        let header = self.header();
        unsafe {
            from_raw_parts(
                (self.0.get() as *const u8).offset(header.session_info_offset as isize),
                header.session_info_len as usize,
            )
        }
    }
}

struct DataValidEvent {
    handle: windows_util::SafeHandle,
}

impl DataValidEvent {
    async fn connect() -> Self {
        let poll_delay = Duration::from_micros(100);
        loop {
            {
                let handle_opt = unsafe {
                    OpenEventA(
                        SYNCHRONIZATION_SYNCHRONIZE,
                        false,
                        PCSTR::from_raw(DATAVALIDEVENTNAME.as_ptr()),
                    )
                }
                .ok()
                .and_then(windows_util::SafeHandle::new);
                if let Some(handle) = handle_opt {
                    return Self { handle };
                }
            }
            tokio::time::sleep(poll_delay).await;
        }
    }

    async fn wait(&self, timeout: Option<Duration>) {
        let handle = unsafe { self.handle.get() };
        let millis = timeout.map_or(INFINITE, |v| v.as_millis() as u32);
        spawn_blocking(move || unsafe {
            WaitForSingleObject(handle, millis);
        })
        .await
        .ok();
    }
}
