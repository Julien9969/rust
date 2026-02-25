use crate::shared::structs::GroupedEntry;
use crate::database::database;

#[tauri::command]
pub async fn get_grouped_data(group_by: String, start_time: i64, end_time: i64) -> Result<Vec<GroupedEntry>, String> {
  match database::get_grouped_entry(group_by, start_time, end_time) {
    Ok(entries) => Ok(entries),
    Err(e) => Err(format!("Error fetching grouped data: {}", e)),
  }
}