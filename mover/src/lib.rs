//! Mover - A Rust implementation of PyAutoGUI
//! 
//! This crate provides cross-platform automation capabilities for controlling
//! the mouse and keyboard programmatically, similar to PyAutoGUI but written in Rust.

pub use mover_core::*;
pub use mover_mouse::{
    position, size, on_screen, move_to, move_by, click, left_click, right_click, middle_click,
    double_click, triple_click, drag_to, drag_by, scroll, vscroll, hscroll
};

pub use mover_keyboard::{
    type_string, type_string_with_interval, press_key, release_key, tap_key, press_keys, press_hotkey
};
pub use mover_screen::{Screen, screen, capture, save_screenshot, get_pixel_color};
pub use mover_utils::{MoverConfig, FailsafeManager, ActionRecorder, sleep, countdown, get_point_on_line, distance_between_points, point_in_rect};
pub use mover_macros::*;

/// Re-export commonly used items for convenience
pub mod prelude {
    pub use mover_core::prelude::*;
    pub use mover_mouse::{Mouse, mouse, position, move_to, move_by, click, left_click, right_click, middle_click, double_click, triple_click, drag_to, drag_by, scroll, vscroll, hscroll};
    
    pub use mover_screen::{Screen, screen, capture, save_screenshot, get_pixel_color};
    pub use mover_utils::{MoverConfig, FailsafeManager, ActionRecorder, sleep, countdown, get_point_on_line, distance_between_points, point_in_rect};
}

/// Initialize the mover library
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Check platform support
    let platform = mover_core::platform::get_platform()?;
    log::info!("Initialized mover on platform: {}", platform.name());
    
    Ok(())
}

/// Get library version
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Get library information
pub fn info() -> String {
    format!(
        "Mover v{} - Rust implementation of PyAutoGUI\nPlatform: {}",
        version(),
        std::env::consts::OS
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }

    #[test]
    fn test_info() {
        let info = info();
        assert!(info.contains("Mover"));
        assert!(info.contains("Rust implementation"));
    }
}
