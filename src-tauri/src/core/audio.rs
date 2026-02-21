use log::{debug, error, warn};
use windows::Win32::Foundation::{CloseHandle, MAX_PATH};
use windows::Win32::Media::Audio::{
    eConsole, eRender, AudioSessionStateActive, IAudioSessionControl,
    IAudioSessionControl2, IAudioSessionManager2, IMMDeviceEnumerator,
    MMDeviceEnumerator,
};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::core::{Interface, PWSTR};

/// Returns a list of application names that are currently outputting audio.
pub fn get_audio_playing_apps() -> Vec<String> {
    match get_audio_playing_apps_inner() {
        Ok(apps) => apps,
        Err(e) => {
            error!("Failed to get audio playing apps: {}", e);
            Vec::new()
        }
    }
}

fn get_audio_playing_apps_inner() -> windows::core::Result<Vec<String>> {
    unsafe {
        // Initialize COM on this thread (no-op if already initialized with same model)
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        // Get the default audio render (output) device
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;

        // Enumerate all audio sessions on the device
        let session_manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None)?;
        let session_enumerator = session_manager.GetSessionEnumerator()?;
        let count = session_enumerator.GetCount()?;

        let mut apps: Vec<String> = Vec::new();

        for i in 0..count {
            let session: IAudioSessionControl = session_enumerator.GetSession(i)?;
            let state = session.GetState()?;

            if state == AudioSessionStateActive {
                let session2: IAudioSessionControl2 = session.cast()?;
                let pid = session2.GetProcessId()?;

                if pid == 0 {
                    // PID 0 = system sounds, skip
                    continue;
                }

                match process_name_from_pid(pid) {
                    Some(name) => {
                        debug!("Audio playing: {} (PID: {})", name, pid);
                        if !apps.contains(&name) {
                            apps.push(name);
                        }
                    }
                    None => {
                        warn!("Could not resolve process name for PID {}", pid);
                    }
                }
            }
        }

        Ok(apps)
    }
}

/// Resolve a process ID to its executable name (without `.exe` extension).
fn process_name_from_pid(pid: u32) -> Option<String> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buffer = vec![0u16; MAX_PATH as usize];
        let mut size = MAX_PATH;
        
        let result = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0), // Win32 path format
            PWSTR(buffer.as_mut_ptr()),
            &mut size,
        );

        let _ = CloseHandle(handle);
        result.ok()?;

        let path = String::from_utf16_lossy(&buffer[..size as usize]);
        let filename = path.rsplit('\\').next().unwrap_or(&path);
        let name = filename.strip_suffix(".exe").unwrap_or(filename);
        Some(name.to_string())
    }
}
