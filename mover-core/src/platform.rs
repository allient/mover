//! Platform abstraction layer for the mover library

use crate::{MoverResult, Point, Size, MouseButton, TweenFn};

/// Platform-specific mouse operations
pub trait MousePlatform {
    /// Get the current mouse position
    fn get_position(&self) -> MoverResult<Point>;
    
    /// Move the mouse to absolute coordinates
    fn move_to(&self, x: i32, y: i32) -> MoverResult<()>;
    
    /// Move the mouse relative to current position
    fn move_by(&self, dx: i32, dy: i32) -> MoverResult<()>;
    
    /// Move the mouse with tweening animation
    fn move_to_with_tween(&self, x: i32, y: i32, duration: f64, tween: TweenFn) -> MoverResult<()>;
    
    /// Click at current position
    fn click(&self, button: MouseButton) -> MoverResult<()>;
    
    /// Click at specific coordinates
    fn click_at(&self, x: i32, y: i32, button: MouseButton) -> MoverResult<()>;
    
    /// Double click at current position
    fn double_click(&self, button: MouseButton) -> MoverResult<()>;
    
    /// Triple click at current position
    fn triple_click(&self, button: MouseButton) -> MoverResult<()>;
    
    /// Press and hold mouse button
    fn press_button(&self, button: MouseButton) -> MoverResult<()>;
    
    /// Release mouse button
    fn release_button(&self, button: MouseButton) -> MoverResult<()>;
    
    /// Drag mouse to absolute coordinates
    fn drag_to(&self, x: i32, y: i32, button: MouseButton) -> MoverResult<()>;
    
    /// Drag mouse relative to current position
    fn drag_by(&self, dx: i32, dy: i32, button: MouseButton) -> MoverResult<()>;
    
    /// Scroll vertically
    fn scroll_vertical(&self, clicks: i32) -> MoverResult<()>;
    
    /// Scroll horizontally
    fn scroll_horizontal(&self, clicks: i32) -> MoverResult<()>;
}

/// Platform-specific screen operations
pub trait ScreenPlatform {
    /// Get screen size
    fn get_size(&self) -> MoverResult<Size>;
    
    /// Check if coordinates are on screen
    fn is_on_screen(&self, x: i32, y: i32) -> MoverResult<bool>;
    
    /// Take a screenshot
    fn capture_screen(&self) -> MoverResult<Vec<u8>>;
    
    /// Capture a region of the screen
    fn capture_region(&self, x: i32, y: i32, width: u32, height: u32) -> MoverResult<Vec<u8>>;
    
    /// Get pixel color at coordinates
    fn get_pixel_color(&self, x: i32, y: i32) -> MoverResult<(u8, u8, u8)>;
}

/// Platform-specific keyboard operations
pub trait KeyboardPlatform {
    /// Type a string of text
    fn type_string(&self, text: &str) -> MoverResult<()>;
    
    /// Press a key
    fn press_key(&self, key: &str) -> MoverResult<()>;
    
    /// Release a key
    fn release_key(&self, key: &str) -> MoverResult<()>;
    
    /// Press and hold a key
    fn hold_key(&self, key: &str) -> MoverResult<()>;
    
    /// Press multiple keys in sequence
    fn press_keys(&self, keys: &[&str]) -> MoverResult<()>;
    
    /// Press hotkey combination
    fn press_hotkey(&self, keys: &[&str]) -> MoverResult<()>;
}

/// Platform trait that combines all platform operations
pub trait Platform: MousePlatform + ScreenPlatform + KeyboardPlatform {
    /// Get the platform name
    fn name(&self) -> &'static str;
    
    /// Check if the platform supports a specific feature
    fn supports_feature(&self, feature: &str) -> bool;
}

/// Get the current platform implementation
/// 
/// Note: This function currently returns an error as platform implementations
/// are not yet implemented. This is a placeholder for future development.
pub fn get_platform() -> MoverResult<Box<dyn Platform>> {
    #[cfg(target_os = "windows")]
    {
        windows::WindowsPlatform::new()
            .map(|p| Box::new(p) as Box<dyn Platform>)
    }
    
    #[cfg(target_os = "macos")]
    {
        macos::MacOSPlatform::new()
            .map(|p| Box::new(p) as Box<dyn Platform>)
    }
    
    #[cfg(target_os = "linux")]
    {
        linux::LinuxPlatform::new()
            .map(|p| Box::new(p) as Box<dyn Platform>)
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err(crate::MoverError::PlatformNotSupported(
            std::env::consts::OS.to_string()
        ))
    }
}

/// Platform implementations
/// 
/// These modules contain the actual platform-specific code
/// for Windows, macOS, and Linux. They are currently placeholder implementations.
/// 
/// TODO: Implement actual platform-specific functionality:
/// - windows.rs - Windows API implementation
/// - macos.rs - macOS Core Graphics implementation  
/// - linux.rs - Linux X11/Wayland implementation
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;
