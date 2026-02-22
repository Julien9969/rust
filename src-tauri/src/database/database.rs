use duckdb::{params, Connection, Result};
use log::{info};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::shared::structs::ActivityEntry;

static DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = match initialize_database() {
        Ok(conn) => conn,
        Err(e) => {
            panic!("Failed to initialize database: {}", e);
        }
    };
    Mutex::new(conn)
});

/// TODO save the DB 
pub fn initialize_database() -> Result<Connection> {   
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activitiesRecord (
            start_time TIMESTAMP,
            end_time TIMESTAMP,
            title TEXT,
            process_path TEXT,
            app_name TEXT,
            is_audio_playing BOOLEAN
        )",
        params![],
    )?;
    info!("Database initialized successfully");
    Ok(conn)
}

pub fn insert_activity_entry(entry: &ActivityEntry) -> Result<()> {
    let conn = DB.lock().unwrap();
    conn.execute(
        "INSERT INTO activitiesRecord 
        (start_time, end_time, title, process_path, app_name, is_audio_playing) 
        VALUES (?, ?, ?, ?, ?, ?)",
        params![
            entry.start_time,
            entry.end_time,
            entry.title,
            entry.process_path,
            entry.app_name,
            entry.is_audio_playing,
        ],
    )?;
    Ok(())
}

pub fn print_latest_entry() -> Result<()> {
    let conn = DB.lock().unwrap();

    let latest_entry = conn.query_row(
        "SELECT start_time, end_time, title, process_path, app_name, is_audio_playing
        FROM activitiesRecord ORDER BY start_time DESC LIMIT 1",
        [],
        |row| {
            Ok(ActivityEntry {
                start_time:       row.get(0)?,
                end_time:         row.get(1)?,
                title:            row.get(2)?,
                process_path:     row.get(3)?,
                app_name:         row.get(4)?,
                is_audio_playing: row.get(5)?,
            })
        },
    );

    match latest_entry {
        Ok(entry) => info!("Latest entry: {}", entry),
        Err(e) => info!("No rows found or error: {}", e),
    }

    Ok(())
}