use active_win_pos_rs::get_active_window;
use log::{error, info};
use system_idle_time::get_idle_time;
use tokio::time::{interval, Duration};
use chrono::{DateTime, Utc};
use crate::shared::structs::ActivityEntry;

use tauri::{AppHandle, Emitter, EventTarget};
use serde::{Serialize, Deserialize};

use super::audio::get_audio_playing_apps;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusUpdate {
  app_name: String,
  idle_time: u128,
  audio_playing_apps: Vec<String>,
}

pub fn send_status(app: &AppHandle, status_update: StatusUpdate) {
  app.emit_filter("status-update", status_update, |target| match target {
    EventTarget::WebviewWindow { label } => label == "main",
    _ => false,
  }).unwrap();
}

pub async fn run_collector(app: AppHandle) {
    let mut ticker = interval(Duration::from_secs(2));

    loop {
        log::debug!("tick");
        ticker.tick().await;

        // let idle = idle_time::get_idle_time()
        //     .map(|d| d.as_secs())
        //     .unwrap_or(0);
        let idle_time = get_idle_time_info();

        // 2️⃣ fenêtre active
        let app_name = get_active_window_info();
        let audio_playing_apps = get_audio_playing_apps();
        if let (Some(idle_time), Some(app_name)) = (idle_time, app_name) {
            send_status(
                &app,
                StatusUpdate { app_name, idle_time, audio_playing_apps }
            );
        }
    }
}

fn get_active_window_info() -> Option<String> {
    match get_active_window() {
        Ok(active_window) => {
            info!("active window: {:#?}", active_window);
            Some(active_window.app_name)
        },
        Err(()) => {
            error!("error occurred while getting the active window");
            None
        }
    }
}

fn get_idle_time_info() -> Option<u128> {
    match get_idle_time() {
        Ok(idle_time) => {
            info!("Idle time: {} ms", idle_time.as_millis());
            Some(idle_time.as_millis())
        },
        Err(e) => {
            error!("Error getting idle time: {}", e);
            None
        }
    }
}