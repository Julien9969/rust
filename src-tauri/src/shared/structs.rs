use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEntry {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub title: String,
    pub process_path: PathBuf,
    pub app_name: String,
    pub idle_time: i128,
    pub is_audio_playing: bool,

}

impl fmt::Display for ActivityEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ActivityEntry {{ start_time: {}, end_time: {}, title: {}, process_path: {}, app_name: {}, idle_time: {}, is_audio_playing: {} }}",
            self.start_time,
            self.end_time,
            self.title,
            self.process_path.display(),
            self.app_name,
            self.idle_time,
            self.is_audio_playing
        )
    }
}