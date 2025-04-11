use crate::utils::cp1252_to_string;
use anyhow::{bail, Context, Result};
use serde::Deserialize;

pub fn parse_session_info(raw: &[u8]) -> Result<SessionInfo> {
    let data_string = cp1252_to_string(raw).context("CP1252 decode of session info failed")?;
    
    match serde_yaml::from_str::<SessionInfo>(&data_string) {
        Ok(session_info) => return Ok(session_info),
        Err(e) => println!("Failed to parse as single SessionInfo: {}", e),
    }
    match serde_yaml::from_str::<Vec<SessionInfo>>(&data_string) {
        Ok(mut items) => {
    if items.is_empty() {
        bail!("Session info did not contain any items");
    }
    Ok(items.swap_remove(0))
        },
        Err(e) => {
            println!("Failed to parse as Vec<SessionInfo>: {}", e);
            bail!("Failed to parse session info in either format: {}", e)
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Session {
    pub session_num: i32,
    pub session_laps: String,
    pub session_time: String,
    pub session_type: String,
}

#[derive(Debug, Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionInfo {
    #[serde(rename = "WeekendInfo")]
    #[serde(default)]
    pub weekend_info: WeekendInfo,
    #[serde(rename = "DriverInfo")]
    #[serde(default)]
    pub driver_info: DriverInfo,
    #[serde(default)]
    pub sessions: Vec<Session>,
}

#[derive(Debug, Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct DriverInfo {
    #[serde(rename="DriverCarSLShiftRPM")]
    pub driver_car_sl_shift_rpm: f64,
    pub driver_pit_trk_pct: String,
    #[serde(rename="DriverCarSLLastRPM")]
    pub driver_car_sl_last_rpm: String,
    pub driver_head_pos_z: String,
    pub driver_car_fuel_max_ltr: String,
    #[serde(rename="DriverCarIdleRPM")]
    pub driver_car_idle_rpm: String,
    pub driver_car_est_lap_time: String,
    pub driver_setup_passed_tech: String,
    pub driver_head_pos_x: String,
    pub driver_car_idx: i64,
    pub driver_setup_is_modified: String,
    #[serde(rename="DriverCarSLBlinkRPM")]
    pub driver_car_sl_blink_rpm: String,
    pub driver_setup_load_type_name: String,
    pub driver_car_red_line: f64,
    pub driver_car_fuel_kg_per_ltr: String,
    #[serde(rename="DriverCarSLFirstRPM")]
    pub driver_car_sl_first_rpm: String,
    pub driver_setup_name: String,
    pub driver_incident_count: String,
    #[serde(rename="DriverUserID")]
    pub driver_user_id: i64,
    pub driver_head_pos_y: String,
    pub pace_car_idx: String,
    pub driver_car_max_fuel_pct: String,
    pub drivers: Vec<Driver>,
}

#[derive(Debug, Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct Driver {
    pub abbrev_name: String,
    pub car_class_color: String,
    #[serde(rename="CarClassID")]
    pub car_class_id: String,
    pub car_class_license_level: String,
    pub car_class_max_fuel_pct: String,
    pub car_class_rel_speed: String,
    pub car_class_short_name: String,
    pub car_class_weight_penalty: String,
    pub car_design_str: String,
    #[serde(rename="CarID")]
    pub car_id: i64,
    pub car_idx: i64,
    #[serde(rename="CarIsAI")]
    pub car_is_ai: String,
    pub car_is_pace_car: String,
    pub car_number: String,
    pub car_number_design_str: String,
    pub car_number_raw: String,
    pub car_path: String,
    pub car_screen_name: String,
    pub car_screen_name_short: String,
    #[serde(rename="CarSponsor_1")]
    pub car_sponsor_1: String,
    #[serde(rename="CarSponsor_2")]
    pub car_sponsor_2: String,
    pub club_name: String,
    pub cur_driver_incident_count: String,
    pub division_name: String,
    pub helmet_design_str: String,
    pub i_rating: String,
    pub initials: String,
    pub is_spectator: String,
    pub lic_color: String,
    pub lic_level: String,
    pub lic_string: String,
    pub lic_sub_level: String,
    pub suit_design_str: String,
    #[serde(rename="TeamID")]
    pub team_id: String,
    pub team_incident_count: String,
    pub team_name: String,
    #[serde(rename="UserID")]
    pub user_id: String,
    pub user_name: String,
} 

#[derive(Debug, Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[serde(default)]
pub struct WeekendInfo {
    #[serde(rename = "SessionID")]
    pub session_id: i32,
    #[serde(rename = "TrackName")]
    pub track_name: String,
    #[serde(rename = "TrackID")]
    pub track_id: String,
    #[serde(rename = "TrackLength")]
    pub track_length: String,
    #[serde(rename = "TrackLengthOfficial")]
    pub track_length_official: String,
    #[serde(rename = "TrackDisplayName")]
    pub track_display_name: String,
    #[serde(rename = "TrackDisplayShortName")]
    pub track_display_short_name: String,
    #[serde(rename = "TrackConfigName")]
    pub track_config_name: String,
    #[serde(rename = "TrackCity")]
    pub track_city: String,
    #[serde(rename = "TrackCountry")]
    pub track_country: String,
    #[serde(rename = "TrackAltitude")]
    pub track_altitude: String,
    #[serde(rename = "TrackLatitude")]
    pub track_latitude: String,
    #[serde(rename = "TrackLongitude")]
    pub track_longitude: String,
    #[serde(rename = "TrackNorthOffset")]
    pub track_north_offset: String,
    #[serde(rename = "TrackNumTurns")]
    pub track_num_turns: i32,
    #[serde(rename = "TrackPitSpeedLimit")]
    pub track_pit_speed_limit: String,
    #[serde(rename = "TrackType")]
    pub track_type: String,
    #[serde(rename = "TrackDirection")]
    pub track_direction: String,
    #[serde(rename = "TrackWeatherType")]
    pub track_weather_type: String,
    #[serde(rename = "TrackSkies")]
    pub track_skies: String,
    #[serde(rename = "TrackSurfaceTemp")]
    pub track_surface_temp: String,
    #[serde(rename = "TrackAirTemp")]
    pub track_air_temp: String,
    #[serde(rename = "TrackAirPressure")]
    pub track_air_pressure: String,
    #[serde(rename = "TrackWindVel")]
    pub track_wind_vel: String,
    #[serde(rename = "TrackWindDir")]
    pub track_wind_dir: String,
    #[serde(rename = "TrackRelativeHumidity")]
    pub track_relative_humidity: String,
    #[serde(rename = "TrackFogLevel")]
    pub track_fog_level: String,
    #[serde(rename = "TrackPrecipitation")]
    pub track_precipitation: String,
    #[serde(rename = "TrackCleanup")]
    pub track_cleanup: String,
    #[serde(rename = "TrackDynamicTrack")]
    pub track_dynamic_track: String,
    #[serde(rename = "TrackVersion")]
    pub track_version: String,
    #[serde(rename = "SeriesID")]
    pub series_id: String,
    #[serde(rename = "SeasonID")]
    pub season_id: String,
    #[serde(rename = "SubSessionID")]
    pub sub_session_id: String,
    #[serde(rename = "LeagueID")]
    pub league_id: String,
    #[serde(rename = "Official")]
    pub official: String,
    #[serde(rename = "RaceWeek")]
    pub race_week: String,
    #[serde(rename = "EventType")]
    pub event_type: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "SimMode")]
    pub sim_mode: String,
    #[serde(rename = "TeamRacing")]
    pub team_racing: String,
    #[serde(rename = "MinDrivers")]
    pub min_drivers: String,
    #[serde(rename = "MaxDrivers")]
    pub max_drivers: String,
    #[serde(rename = "DCRuleSet")]
    pub dc_rule_set: String,
    #[serde(rename = "QualifierMustStartRace")]
    pub qualifier_must_start_race: String,
    #[serde(rename = "NumCarClasses")]
    pub num_car_classes: String,
    #[serde(rename = "NumCarTypes")]
    pub num_car_types: String,
    #[serde(rename = "HeatRacing")]
    pub heat_racing: String,
    #[serde(rename = "BuildType")]
    pub build_type: String,
    #[serde(rename = "BuildTarget")]
    pub build_target: String,
    #[serde(rename = "BuildVersion")]
    pub build_version: String,
    #[serde(rename = "RaceFarm")]
    pub race_farm: String,
    #[serde(rename = "WeekendOptions")]
    #[serde(default)]
    pub weekend_options: WeekendOptions,
}

#[derive(Debug, Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WeekendOptions {
    #[serde(rename = "NumStarters")]
    pub num_starters: String,
    #[serde(rename = "StartingGrid")]
    pub starting_grid: String,
    #[serde(rename = "QualifyScoring")]
    pub qualify_scoring: String,
    #[serde(rename = "CourseCautions")]
    pub course_cautions: String,
    #[serde(rename = "StandingStart")]
    pub standing_start: String,
    #[serde(rename = "ShortParadeLap")]
    pub short_parade_lap: String,
    #[serde(rename = "Restarts")]
    pub restarts: String,
    #[serde(rename = "WeatherType")]
    pub weather_type: String,
    #[serde(rename = "Skies")]
    pub skies: String,
    #[serde(rename = "WindDirection")]
    pub wind_direction: String,
    #[serde(rename = "WindSpeed")]
    pub wind_speed: String,
    #[serde(rename = "WeatherTemp")]
    pub weather_temp: String,
    #[serde(rename = "RelativeHumidity")]
    pub relative_humidity: String,
    #[serde(rename = "FogLevel")]
    pub fog_level: String,
    #[serde(rename = "TimeOfDay")]
    pub time_of_day: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "EarthRotationSpeedupFactor")]
    pub earth_rotation_speedup_factor: String,
    #[serde(rename = "Unofficial")]
    pub unofficial: String,
    #[serde(rename = "CommercialMode")]
    pub commercial_mode: String,
    #[serde(rename = "NightMode")]
    pub night_mode: String,
    #[serde(rename = "IsFixedSetup")]
    pub is_fixed_setup: String,
    #[serde(rename = "StrictLapsChecking")]
    pub strict_laps_checking: String,
    #[serde(rename = "HasOpenRegistration")]
    pub has_open_registration: String,
    #[serde(rename = "HardcoreLevel")]
    pub hardcore_level: String,
    #[serde(rename = "NumJokerLaps")]
    pub num_joker_laps: String,
    #[serde(rename = "IncidentLimit")]
    pub incident_limit: String,
    #[serde(rename = "FastRepairsLimit")]
    pub fast_repairs_limit: String,
    #[serde(rename = "GreenWhiteCheckeredLimit")]
    pub green_white_checkered_limit: String,
    #[serde(rename = "TelemetryOptions")]
    #[serde(default)]
    pub telemetry_options: TelemetryOptions,
}

#[derive(Debug, Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TelemetryOptions {
    #[serde(rename = "TelemetryDiskFile")]
    pub telemetry_disk_file: String,
}

impl Default for WeekendInfo {
    fn default() -> Self {
        Self {
            session_id: 0,
            track_name: String::new(),
            track_id: String::new(),
            track_length: String::new(),
            track_length_official: String::new(),
            track_display_name: String::new(),
            track_display_short_name: String::new(),
            track_config_name: String::new(),
            track_city: String::new(),
            track_country: String::new(),
            track_altitude: String::new(),
            track_latitude: String::new(),
            track_longitude: String::new(),
            track_north_offset: String::new(),
            track_num_turns: 0,
            track_pit_speed_limit: String::new(),
            track_type: String::new(),
            track_direction: String::new(),
            track_weather_type: String::new(),
            track_skies: String::new(),
            track_surface_temp: String::new(),
            track_air_temp: String::new(),
            track_air_pressure: String::new(),
            track_wind_vel: String::new(),
            track_wind_dir: String::new(),
            track_relative_humidity: String::new(),
            track_fog_level: String::new(),
            track_precipitation: String::new(),
            track_cleanup: String::new(),
            track_dynamic_track: String::new(),
            track_version: String::new(),
            series_id: String::new(),
            season_id: String::new(),
            sub_session_id: String::new(),
            league_id: String::new(),
            official: String::new(),
            race_week: String::new(),
            event_type: String::new(),
            category: String::new(),
            sim_mode: String::new(),
            team_racing: String::new(),
            min_drivers: String::new(),
            max_drivers: String::new(),
            dc_rule_set: String::new(),
            qualifier_must_start_race: String::new(),
            num_car_classes: String::new(),
            num_car_types: String::new(),
            heat_racing: String::new(),
            build_type: String::new(),
            build_target: String::new(),
            build_version: String::new(),
            race_farm: String::new(),
            weekend_options: WeekendOptions::default(),
        }
    }
}

impl Default for WeekendOptions {
    fn default() -> Self {
        Self {
            num_starters: String::new(),
            starting_grid: String::new(),
            qualify_scoring: String::new(),
            course_cautions: String::new(),
            standing_start: String::new(),
            short_parade_lap: String::new(),
            restarts: String::new(),
            weather_type: String::new(),
            skies: String::new(),
            wind_direction: String::new(),
            wind_speed: String::new(),
            weather_temp: String::new(),
            relative_humidity: String::new(),
            fog_level: String::new(),
            time_of_day: String::new(),
            date: String::new(),
            earth_rotation_speedup_factor: String::new(),
            unofficial: String::new(),
            commercial_mode: String::new(),
            night_mode: String::new(),
            is_fixed_setup: String::new(),
            strict_laps_checking: String::new(),
            has_open_registration: String::new(),
            hardcore_level: String::new(),
            num_joker_laps: String::new(),
            incident_limit: String::new(),
            fast_repairs_limit: String::new(),
            green_white_checkered_limit: String::new(),
            telemetry_options: TelemetryOptions::default(),
        }
    }
}

impl Default for TelemetryOptions {
    fn default() -> Self {
        Self {
            telemetry_disk_file: String::new(),
        }
    }
}

impl Default for DriverInfo {
    fn default() -> Self {
        Self {
            driver_car_sl_shift_rpm: 0.0,
            driver_pit_trk_pct: String::new(),
            driver_car_sl_last_rpm: String::new(),
            driver_head_pos_z: String::new(),
            driver_car_fuel_max_ltr: String::new(),
            driver_car_idle_rpm: String::new(),
            driver_car_est_lap_time: String::new(),
            driver_setup_passed_tech: String::new(),
            driver_head_pos_x: String::new(),
            driver_car_idx: 0,
            driver_setup_is_modified: String::new(),
            driver_car_sl_blink_rpm: String::new(),
            driver_setup_load_type_name: String::new(),
            driver_car_red_line: 0.0,
            driver_car_fuel_kg_per_ltr: String::new(),
            driver_car_sl_first_rpm: String::new(),
            driver_setup_name: String::new(),
            driver_incident_count: String::new(),
            driver_user_id: 0,
            driver_head_pos_y: String::new(),
            pace_car_idx: String::new(),
            driver_car_max_fuel_pct: String::new(),
            drivers: Vec::new(),
        }
    }
}

impl Default for Driver {
    fn default() -> Self {
        Self {
            abbrev_name: String::new(),
            car_class_color: String::new(),
            car_class_id: String::new(),
            car_class_license_level: String::new(),
            car_class_max_fuel_pct: String::new(),
            car_class_rel_speed: String::new(),
            car_class_short_name: String::new(),
            car_class_weight_penalty: String::new(),
            car_design_str: String::new(),
            car_id: 0,
            car_idx: 0,
            car_is_ai: String::new(),
            car_is_pace_car: String::new(),
            car_number: String::new(),
            car_number_design_str: String::new(),
            car_number_raw: String::new(),
            car_path: String::new(),
            car_screen_name: String::new(),
            car_screen_name_short: String::new(),
            car_sponsor_1: String::new(),
            car_sponsor_2: String::new(),
            club_name: String::new(),
            cur_driver_incident_count: String::new(),
            division_name: String::new(),
            helmet_design_str: String::new(),
            i_rating: String::new(),
            initials: String::new(),
            is_spectator: String::new(),
            lic_color: String::new(),
            lic_level: String::new(),
            lic_string: String::new(),
            lic_sub_level: String::new(),
            suit_design_str: String::new(),
            team_id: String::new(),
            team_incident_count: String::new(),
            team_name: String::new(),
            user_id: String::new(),
            user_name: String::new(),
        }
    }
}

