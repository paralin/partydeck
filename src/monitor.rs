use x11rb::connection::Connection;
use x11rb::protocol::randr::ConnectionExt as _;

use std::fs;
use std::path::Path;


#[derive(Clone)]
pub struct Monitor {
    name: String,
    width: u32,
    height: u32,
}

impl Monitor {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

// This should mimic the SDL monitor retrival used by gamescope, while avoiding all of SDL. (IGNORES SDL_HINT_VIDEO_DISPLAY_PRIORITY, and if display dosnt have "visual info" because all modern one will)
// https://github.com/libsdl-org/SDL/blob/225fb12ae13b70689bcb8c0b42bf061120fefcc4/src/video/x11/SDL_x11modes.c#L868
fn get_monitors_x11() -> Result<Vec<Monitor>, Box<dyn std::error::Error>> {
    let (con, screen_num) = x11rb::connect(None)?;
    let screen = &con.setup().roots[screen_num];

    // Get primary output (sorted first in sdl, but as sdl comments say, this should be done already.)
    let primary = con
        .randr_get_output_primary(screen.root)?
        .reply()?
        .output;

    let res = con
        .randr_get_screen_resources(screen.root)?
        .reply()?;

    let mut monitors = Vec::new();

    for output in &res.outputs {
        let info = con
            .randr_get_output_info(*output, res.config_timestamp)?
            .reply()?;

        if info.connection != x11rb::protocol::randr::Connection::CONNECTED || info.crtc == 0 {
            continue;
        }

        let crtc = con
            .randr_get_crtc_info(info.crtc, res.config_timestamp)?
            .reply()?;

        let name = String::from_utf8_lossy(&info.name).to_string();

        let monitor = Monitor {
            name: name.clone(),
            width: crtc.width.into(),
            height: crtc.height.into(),
        };

        if *output == primary {
            // Insert primary at the front (SDL requirement for some reason)
            monitors.insert(0, monitor);
        } else {
            monitors.push(monitor);
        }
    }

    Ok(monitors)
}

/// Detect monitors via DRM/KMS sysfs. Works on both X11 and Wayland,
/// and does not require a display connection (works over SSH too).
/// Reads /sys/class/drm/card*-*/status and /sys/class/drm/card*-*/modes.
fn get_monitors_drm() -> Result<Vec<Monitor>, Box<dyn std::error::Error>> {
    let drm_dir = Path::new("/sys/class/drm");
    if !drm_dir.exists() {
        return Err("/sys/class/drm not found".into());
    }

    let mut monitors = Vec::new();

    let mut entries: Vec<_> = fs::read_dir(drm_dir)?
        .filter_map(|e| e.ok())
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();

        // Only look at connector entries (card0-HDMI-A-1, card0-DP-1, etc.)
        // Skip bare card entries, renderD*, and Writeback connectors.
        if !name.contains('-') || name.contains("Writeback") {
            continue;
        }

        let path = entry.path();

        let status = fs::read_to_string(path.join("status"))
            .unwrap_or_default();
        if status.trim() != "connected" {
            continue;
        }

        let modes = fs::read_to_string(path.join("modes"))
            .unwrap_or_default();

        // First line is the current/preferred mode, e.g. "3840x2160"
        if let Some(first_mode) = modes.lines().next() {
            if let Some((w, h)) = first_mode.split_once('x') {
                if let (Ok(width), Ok(height)) = (w.trim().parse::<u32>(), h.trim().parse::<u32>()) {
                    // Strip the "card0-" prefix for a cleaner name
                    let connector_name = name.split_once('-')
                        .map(|(_, rest)| rest)
                        .unwrap_or(&name)
                        .to_string();

                    monitors.push(Monitor {
                        name: connector_name,
                        width,
                        height,
                    });
                }
            }
        }
    }

    Ok(monitors)
}

pub fn get_monitors_errorless() -> Vec<Monitor> {
    // Try X11/RandR first (matches gamescope's SDL behavior)
    if let Ok(monitors) = get_monitors_x11() {
        if !monitors.is_empty() {
            return monitors;
        }
    }

    // Fall back to DRM/KMS sysfs (works on Wayland and over SSH)
    println!("[PARTYDECK] X11 monitor detection failed, trying DRM/KMS sysfs...");
    if let Ok(monitors) = get_monitors_drm() {
        if !monitors.is_empty() {
            for m in &monitors {
                println!("[PARTYDECK] DRM: {} ({}x{})", m.name, m.width, m.height);
            }
            return monitors;
        }
    }

    println!("[PARTYDECK] All monitor detection failed; using assumed 1920x1080");
    vec![Monitor {
        name: "Partydeck Virtual Monitor".to_string(),
        width: 1920,
        height: 1080,
    }]
}
