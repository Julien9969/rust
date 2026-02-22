use chrono::{DateTime, Utc};
use std::fmt;

pub struct ActivityEntry {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub title: String,
    pub process_path: String,
    pub app_name: String,
    pub is_audio_playing: bool,

}

impl fmt::Display for ActivityEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ActivityEntry {{ start_time: {}, end_time: {}, title: {}, process_path: {}, app_name: {}, is_audio_playing: {} }}",
            self.start_time,
            self.end_time,
            self.title,
            self.process_path,
            self.app_name,
            self.is_audio_playing
        )
    }
}