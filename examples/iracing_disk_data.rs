fn main() {
    use simetry::iracing::DiskClient;
    use std::env;
    use std::thread::sleep;
    use std::time::Duration;

    let mut client =
        DiskClient::open(env::args().nth(1).expect("Filename argument required")).unwrap();
    let mut vars: Vec<_> = client.variables().into_iter().collect();
    vars.sort_by(|(a,_), (b,_)| a.cmp(b));
    for (key, val) in vars {
        println!("{}: {:?}", key, val);
    }
    println!("Session info: {:?}", client.session_info());
    sleep(Duration::from_millis(500));
    while let Some(sim_state) = client.latest_sim_state() {
        let gear = sim_state.read_name("Gear").unwrap_or(0);
        let rpm = f32::round(sim_state.read_name("RPM").unwrap_or(0.0));
        let speed = f32::round(sim_state.read_name("Speed").unwrap_or(0.0));
        let lat = sim_state.read_name("Lat").unwrap_or(0.0);
        let lon = sim_state.read_name("Lon").unwrap_or(0.0);
        let alt = f32::round(sim_state.read_name("Alt").unwrap_or(0.0));
        print!("{:<12} {:>3} m/s @ {:>5} RPM, {:>3.8}°, {:>3.8}°, {:>4}m, {:>1} gear              \r", sim_state.tick(), speed, rpm, lat, lon, alt, gear);
        sleep(Duration::from_millis(16));
    }
}