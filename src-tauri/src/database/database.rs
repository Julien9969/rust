use duckdb::{params, Connection, Result};
use log::{warn, info, debug};
use std::{path::PathBuf, sync::Mutex};
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
fn initialize_database() -> Result<Connection> {   
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activityRecords (
            start_time TIMESTAMP,
            end_time TIMESTAMP,
            title TEXT,
            process_path TEXT,
            app_name TEXT,
            idle_time BIGINT,
            is_audio_playing BOOLEAN
        )",
        params![],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_start_time ON activityRecords(start_time)",
        params![],
    )?;
    info!("Database initialized successfully");
    Ok(conn)
}

pub fn insert_activity_entry(entry: &ActivityEntry) -> Result<()> {
    let conn = DB.lock().unwrap();
    conn.execute(
        "INSERT INTO activityRecords 
        (start_time, end_time, title, process_path, app_name, idle_time, is_audio_playing) 
        VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            entry.start_time,
            entry.end_time,
            entry.title,
            entry.process_path.to_str().unwrap_or_default(),
            entry.app_name,
            entry.idle_time,
            entry.is_audio_playing,
        ],
    )?;
    Ok(())
}

pub fn get_latest_entry() -> Result<Option<ActivityEntry>> {
    let conn = DB.lock().unwrap();

    let latest_entry = conn.query_row(
        "SELECT start_time, end_time, title, process_path, app_name, idle_time, is_audio_playing
        FROM activityRecords ORDER BY start_time DESC LIMIT 1",
        [],
        |row| {
            Ok(ActivityEntry {
                start_time:       row.get(0)?,
                end_time:         row.get(1)?,
                title:            row.get(2)?,
                process_path:     PathBuf::from(row.get::<_, String>(3)?),
                app_name:         row.get(4)?,
                idle_time:        row.get(5)?,
                is_audio_playing: row.get(6)?,
            })
        },
    );

    match latest_entry {
        Ok(entry) => Ok(Some(entry)),
        Err(duckdb::Error::QueryReturnedNoRows) => {
            debug!("Table is empty");
            Ok(None)
        },
        Err(e) => {
            warn!("No rows found or error: {}", e);
            Err(e)
        }
    }
}

pub fn update_latest_entry(entry: &ActivityEntry) -> () {
    let conn = DB.lock().unwrap();
    let result = conn.execute(
        "UPDATE activityRecords SET 
        end_time = ?, idle_time = ?
        WHERE start_time = (SELECT MAX(start_time) FROM activityRecords)",
        params![
            entry.end_time,
            entry.idle_time,
        ],
    );

    match result {
        Ok(rows_updated) => {
            if rows_updated == 0 {
                debug!("No rows updated, no entry found to update");
            } else {
                debug!("Updated latest entry successfully");
            }
        },
        Err(e) => {
            warn!("Error updating latest entry: {}", e);
        }
    }
}