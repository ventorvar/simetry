use crate::utils::cp1252_to_string;
use anyhow::{bail, Context, Result};

pub fn parse_session_info(raw: &[u8]) -> Result<SessionInfo> {
    let data_string = cp1252_to_string(raw).context("CP1252 decode of session info failed")?;
    let mut items: Vec<SessionInfo> = serde_yaml::from_str(&data_string)?;
    if items.is_empty() {
        bail!("Session info did not contain any items");
    }
    Ok(items.swap_remove(0))
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionInfo {
    pub driver_info: DriverInfo,
    pub weekend_info: WeekendInfo,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DriverInfo {
    #[serde(rename="DriverCarSLShiftRPM")]
    pub driver_car_sl_shift_rpm: f64,
    pub driver_pit_trk_pct: f64,
    #[serde(rename="DriverCarSLLastRPM")]
    pub driver_car_sl_last_rpm: f64,
    pub driver_head_pos_z: f64,
    pub driver_car_fuel_max_ltr: f64,
    #[serde(rename="DriverCarIdleRPM")]
    pub driver_car_idle_rpm: f64,
    pub driver_car_est_lap_time: f64,
    pub driver_setup_passed_tech: f64,
    pub driver_head_pos_x: f64,
    pub driver_car_idx: i64,
    pub driver_setup_is_modified: f64,
    #[serde(rename="DriverCarSLBlinkRPM")]
    pub driver_car_sl_blink_rpm: f64,
    pub driver_setup_load_type_name: String,
    pub driver_car_red_line: f64,
    pub driver_car_fuel_kg_per_ltr: f64,
    #[serde(rename="DriverCarSLFirstRPM")]
    pub driver_car_sl_first_rpm: f64,
    pub driver_setup_name: String,
    pub driver_incident_count: f64,
    pub driver_user_id: f64,
    pub driver_head_pos_y: f64,
    pub pace_car_idx: i64,
    pub driver_car_max_fuel_pct: f64,
    pub drivers: Vec<Driver>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Driver {
    pub abbrev_name: String,
    pub car_class_color: String,
    #[serde(rename="CarClassID")]
    pub car_class_id: i64,
    pub car_class_license_level: u8,
    pub car_class_max_fuel_pct: f64,
    pub car_class_rel_speed: f64,
    pub car_class_short_name: String,
    pub car_class_weight_penalty: u8,
    pub car_design_str: String,
    #[serde(rename="CarID")]
    pub car_id: i64,
    pub car_idx: i64,
    #[serde(rename="CarIsAI")]
    pub car_is_ai: bool,
    pub car_is_pace_car: bool,
    pub car_number: i64,
    pub car_number_design_str: String,
    pub car_number_raw: u8,
    pub car_path: u8,
    pub car_screen_name: String,
    pub car_screen_name_short: String,
    #[serde(rename="CarSponsor_1")]
    pub car_sponsor_1: u8,
    #[serde(rename="CarSponsor_2")]
    pub car_sponsor_2: u8,
    pub club_name: String,
    pub cur_driver_incident_count: u8,
    pub division_name: String,
    pub helmet_design_str: String,
    pub i_rating: u8,
    pub initials: String,
    pub is_spectator: bool,
    pub lic_color: String,
    pub lic_level: u8,
    pub lic_string: String,
    pub lic_sub_level: u8,
    pub suit_design_str: String,
    #[serde(rename="TeamID")]
    pub team_id: i64,
    pub team_incident_count: i64,
    pub team_name: String,
    #[serde(rename="UserID")]
    pub user_id: i64,
    pub user_name: String,
} 

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeekendInfo {
    pub session_id: String,
}

