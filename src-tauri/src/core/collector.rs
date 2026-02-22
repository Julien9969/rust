use active_win_pos_rs::{get_active_window, ActiveWindow};
use log::{error, info, trace};
use system_idle_time::get_idle_time;
use tokio::time::{interval, Duration};
use chrono::{Utc};
use crate::database::database::{get_latest_entry, update_latest_entry,insert_activity_entry};
use crate::shared::structs::{ActivityEntry};

use tauri::{AppHandle, Emitter, EventTarget};

use super::audio::get_audio_playing_apps;

pub fn send_status(app: &AppHandle, status_update: ActivityEntry) {
  app.emit_filter("status-update", status_update, |target| match target {
    EventTarget::WebviewWindow { label } => label == "main",
    _ => false,
  }).unwrap();
}

pub async fn run_collector(app: AppHandle) {
    let mut ticker = interval(Duration::from_secs(5));

    loop {
        ticker.tick().await;
       
        let idle_time = get_idle_time_info();
        let window_infos = get_active_window_infos();

        let audio_playing_apps = get_audio_playing_apps();
        info!("Audio playing {} : {:?}", audio_playing_apps.iter().count(), audio_playing_apps);
        if let (Some(idle_time), Some(window_infos)) = (idle_time, window_infos) {
            match get_latest_entry() {
                Ok(Some(mut entry)) => {
                    if entry.app_name == window_infos.app_name
                    {
                        entry.end_time = Utc::now();
                        entry.idle_time = idle_time as i128; //TODO handle active -> idle -> active scenario
                        update_latest_entry(&entry);
                        info!("Updated latest entry: {}", entry);
                    }
                    else 
                    {
                        entry = ActivityEntry {
                            start_time: Utc::now(),
                            end_time: Utc::now(),
                            title: window_infos.title,
                            process_path: window_infos.process_path,
                            app_name: window_infos.app_name.clone(),
                            idle_time: 0i128,
                            is_audio_playing: audio_playing_apps.iter().count() > 0,
                        };
                        insert_activity_entry(&entry).unwrap();
                        info!("Inserted new entry: {}", entry);
                    }
                    send_status(&app, entry.clone());
                }
                Ok(None) => {
                    let entry = ActivityEntry {
                        start_time: Utc::now(),
                        end_time: Utc::now(),
                        title: window_infos.title,
                        process_path: window_infos.process_path,
                        app_name: window_infos.app_name.clone(),
                        idle_time: 0i128,
                        is_audio_playing: audio_playing_apps.iter().count() > 0,
                    };
                    insert_activity_entry(&entry).unwrap();
                    info!("Inserted new entry: {}", entry);
                }
                Err(e) => {
                    error!("Error getting latest entry: {}", e);
                }
            }
        }
    }
}

fn get_active_window_infos() -> Option<ActiveWindow> {
    match get_active_window() {
        Ok(active_window) => {
            trace!("active window: {:#?}", active_window);
            Some(active_window)
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
            trace!("Idle time: {} ms", idle_time.as_millis());
            Some(idle_time.as_millis())
        },
        Err(e) => {
            error!("Error getting idle time: {}", e);
            None
        }
    }
}