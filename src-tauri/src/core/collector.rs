use active_win_pos_rs::get_active_window;
use log::{error, info};
use system_idle_time::get_idle_time;
use tokio::time::{interval, Duration};


pub async fn run_collector(
    // idle_threshold: u64,
) {
    let mut ticker = interval(Duration::from_secs(5));

    loop {
        log::debug!("tick");
        ticker.tick().await;

        // let idle = idle_time::get_idle_time()
        //     .map(|d| d.as_secs())
        //     .unwrap_or(0);
        get_idle_time_info();

        // 2️⃣ fenêtre active
        get_active_window_info();
    }
}


fn get_active_window_info() {
    match get_active_window() {
        Ok(active_window) => {
            info!("active window: {:#?}", active_window);
        },
        Err(()) => {
            error!("error occurred while getting the active window");
        }
    }
}

fn get_idle_time_info() {
    match get_idle_time() {
        Ok(idle_time) => info!("Idle time: {} ms", idle_time.as_millis()),
        Err(e) => error!("Error getting idle time: {}", e),
    }
}