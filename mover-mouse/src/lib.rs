//! Mouse control module for the mover automation library
//! 
//! This module provides comprehensive mouse control functionality similar to PyAutoGUI,
//! including movement, clicking, dragging, and scrolling.
//! 
//! Uses the `enigo` crate for cross-platform mouse control implementation.
//! 
//! # Features
//! 
//! - **Cross-Platform Support**: Works on Windows, macOS, and Linux
//! - **Real Mouse Control**: Actual mouse movements, clicks, and scrolling
//! - **Smooth Animations**: Tweening functions for natural mouse movements
//! - **Comprehensive API**: All the mouse functions you need for automation
//! - **Type Safety**: Rust's type system ensures safe mouse operations
//! - **Error Handling**: Proper error handling with `Result` types
//! 
//! # Quick Start
//! 
//! ```rust
//! use mover_mouse::Mouse;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a mouse instance
//!     let mouse = Mouse::new()?;
//!     
//!     // Get current position
//!     let pos = mouse.position()?;
//!     
//!     // Move to center of screen
//!     let screen = mouse.size()?;
//!     mouse.move_to(screen.width / 2, screen.height / 2)?;
//!     
//!     // Click
//!     mouse.left_click()?;
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Smooth Mouse Movement
//! 
//! ```rust
//! use mover_mouse::Mouse;
//! use mover_core::ease_in_out_quad;
//! 
//! fn smooth_mouse_movement() -> Result<(), Box<dyn std::error::Error>> {
//!     let mouse = Mouse::new()?;
//!     // Move with smooth animation over 2 seconds
//!     mouse.move_to_with_tween(500, 300, 2.0, ease_in_out_quad)?;
//!     Ok(())
//! }
//! ```
//! 
//! ## Mouse Dragging
//! 
//! ```rust
//! use mover_mouse::Mouse;
//! use mover_core::types::MouseButton;
//! 
//! fn mouse_dragging() -> Result<(), Box<dyn std::error::Error>> {
//!     let mouse = Mouse::new()?;
//!     // Drag from current position to (300, 400)
//!     mouse.drag_to(300, 400, Some(MouseButton::Left))?;
//!     Ok(())
//! }
//! ```

use mover_core::{MoverResult, Point, Size, TweenFn};
use enigo::{Button as EnigoMouseButton, Enigo, Settings, Direction, Coordinate, Axis, Mouse as EnigoMouse};
use std::{thread, time::Duration};
use std::io::Write;

// Re-export commonly used types for convenience
pub use mover_core::MouseButton;

/// Mouse control interface providing comprehensive automation capabilities.
/// 
/// This struct contains all the methods needed for mouse control, including:
/// - Position tracking and screen information
/// - Mouse movement (absolute and relative)
/// - Clicking (single, double, triple)
/// - Dragging with button states
/// - Scrolling (vertical and horizontal)
/// - Utility functions for automation
/// 
/// # Examples
/// 
/// ```rust
/// use mover_mouse::Mouse;
/// 
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a mouse instance
///     let mouse = Mouse::new()?;
///     
///     // Get current position
///     let pos = mouse.position()?;
/// 
///     // Move to specific coordinates
///     mouse.move_to(100, 200)?;
/// 
///     // Click with left button
///     mouse.left_click()?;
///     Ok(())
/// }
/// ```
pub struct Mouse {
    enigo: Enigo,
}

impl Mouse {
    /// Create a new Mouse instance
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let enigo = Enigo::new(&Settings::default())
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to create Enigo instance: {}", e)
                )
            ))?;
        Ok(Mouse { enigo })
    }

    // Position and Information Functions
    // =================================
    
    /// Returns the current xy coordinates of the mouse cursor.
    /// 
    /// This function queries the operating system for the current mouse position
    /// and returns it as a `Point` struct.
    /// 
    /// # Returns
    /// 
    /// - `Ok(Point)` - The current mouse coordinates
    /// - `Err(MoverError)` - If the position cannot be retrieved
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_mouse::Mouse;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mouse = Mouse::new()?;
    ///     let pos = mouse.position()?;
    ///     println!("Mouse is at: ({}, {})", pos.x, pos.y);
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Platform Notes
    /// 
    /// - **Windows**: Uses Windows API to get cursor position
    /// - **macOS**: Uses Core Graphics to get cursor position  
    /// - **Linux**: Uses X11 to get cursor position
    pub fn position(&self) -> MoverResult<Point> {
        let (x, y) = self.enigo.location()
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to get mouse position: {}", e)
                )
            ))?;
        Ok(Point::new(x, y))
    }
    
    /// Returns the width and height of the primary display.
    /// 
    /// This function queries the operating system for the screen dimensions
    /// and returns them as a `Size` struct.
    /// 
    /// # Returns
    /// 
    /// - `Ok(Size)` - The screen dimensions (width × height)
    /// - `Err(MoverError)` - If the screen size cannot be retrieved
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_mouse::Mouse;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mouse = Mouse::new()?;
    ///     let screen = mouse.size()?;
    ///     println!("Screen resolution: {}x{}", screen.width, screen.height);
    /// 
    ///     // Calculate center of screen
    ///     let center_x = screen.width / 2;
    ///     let center_y = screen.height / 2;
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Platform Notes
    /// 
    /// - **Windows**: Uses Windows API to get screen dimensions
    /// - **macOS**: Uses Core Graphics to get screen dimensions
    /// - **Linux**: Uses X11 to get screen dimensions
    pub fn size(&self) -> MoverResult<Size> {
        let (width, height) = self.enigo.main_display()
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to get screen size: {}", e)
                )
            ))?;
        Ok(Size::new(width, height))
    }
    
    /// Returns whether the given coordinates are on the screen.
    /// 
    /// This function checks if the specified coordinates fall within
    /// the bounds of the primary display.
    /// 
    /// # Arguments
    /// 
    /// * `x` - The x coordinate to check
    /// * `y` - The y coordinate to check
    /// 
    /// # Returns
    /// 
    /// - `Ok(bool)` - `true` if coordinates are on screen, `false` otherwise
    /// - `Err(MoverError)` - If the check cannot be performed
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_mouse::Mouse;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mouse = Mouse::new()?;
    ///     
    ///     // Check if (100, 200) is on screen
    ///     let on_screen = mouse.on_screen(100, 200)?;
    ///     println!("Point (100, 200) is on screen: {}", on_screen);
    ///     
    ///     // Check if (-100, -200) is on screen (should be false)
    ///     let on_screen = mouse.on_screen(-100, -200)?;
    ///     println!("Point (-100, -200) is on screen: {}", on_screen);
    ///     Ok(())
    /// }
    /// ```
    pub fn on_screen(&self, x: i32, y: i32) -> MoverResult<bool> {
        let screen = self.size()?;
        Ok(x >= 0 && y >= 0 && x < screen.width && y < screen.height)
    }

    // Movement Functions
    // ==================
    
    /// Moves the mouse cursor to the given x and y coordinates.
    /// 
    /// This function moves the mouse cursor to the specified absolute coordinates
    /// on the screen. The movement is instant (no animation).
    /// 
    /// # Arguments
    /// 
    /// * `x` - The target x coordinate
    /// * `y` - The target y coordinate
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - Mouse moved successfully
    /// - `Err(MoverError)` - If the movement failed
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_mouse::Mouse;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mouse = Mouse::new()?;
    ///     
    ///     // Move to specific coordinates
    ///     mouse.move_to(500, 300)?;
    ///     
    ///     // Move to center of screen
    ///     let screen = mouse.size()?;
    ///     mouse.move_to(screen.width / 2, screen.height / 2)?;
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Platform Notes
    /// 
    /// - **Windows**: Uses Windows API to move cursor
    /// - **macOS**: Uses Core Graphics to move cursor
    /// - **Linux**: Uses X11 to move cursor
    pub fn move_to(&mut self, x: i32, y: i32) -> MoverResult<()> {
        self.enigo.move_mouse(x, y, Coordinate::Abs)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to move mouse: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Moves the mouse cursor to a point on the screen with animation.
    /// 
    /// This function moves the mouse cursor to the specified coordinates
    /// over the given duration using linear interpolation.
    /// 
    /// # Arguments
    /// 
    /// * `x` - The target x coordinate
    /// * `y` - The target y coordinate
    /// * `duration` - The duration of the movement in seconds
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - Mouse moved successfully
    /// - `Err(MoverError)` - If the movement failed
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_mouse::Mouse;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mouse = Mouse::new()?;
    ///     
    ///     // Move to (500, 300) over 2 seconds
    ///     mouse.move_to_with_duration(500, 300, 2.0)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn move_to_with_duration(&mut self, x: i32, y: i32, duration: f64) -> MoverResult<()> {
        self.move_to_with_tween(x, y, duration, mover_core::linear_tween)
    }
    
    /// Moves the mouse cursor to a point on the screen with custom tweening animation.
    /// 
    /// This function moves the mouse cursor to the specified coordinates
    /// over the given duration using the provided tweening function.
    /// 
    /// # Arguments
    /// 
    /// * `x` - The target x coordinate
    /// * `y` - The target y coordinate
    /// * `duration` - The duration of the movement in seconds
    /// * `tween` - The tweening function to use for animation
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - Mouse moved successfully
    /// - `Err(MoverError)` - If the movement failed
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_mouse::Mouse;
    /// use mover_core::ease_in_out_quad;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mouse = Mouse::new()?;
    ///     
    ///     // Move with smooth easing
    ///     mouse.move_to_with_tween(500, 300, 2.0, ease_in_out_quad)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn move_to_with_tween(&mut self, x: i32, y: i32, duration: f64, tween: TweenFn) -> MoverResult<()> {
        if duration <= 0.0 {
            return self.move_to(x, y);
        }
        
        let start_pos = self.position()?;
        let steps = (duration * 60.0).max(1.0) as usize; // 60 FPS
        
        for i in 0..=steps {
            let progress = i as f64 / steps as f64;
            let tweened_progress = tween(progress);
            
            let current_x = start_pos.x + ((x - start_pos.x) as f64 * tweened_progress) as i32;
            let current_y = start_pos.y + ((y - start_pos.y) as f64 * tweened_progress) as i32;
            
            self.move_to(current_x, current_y)?;
            
            if i < steps {
                self.sleep(duration / steps as f64);
            }
        }
        
        Ok(())
    }
    
    /// Moves the mouse cursor relative to current position
    pub fn move_by(&mut self, dx: i32, dy: i32) -> MoverResult<()> {
        let current_pos = self.position()?;
        let target_x = current_pos.x + dx;
        let target_y = current_pos.y + dy;
        self.move_to(target_x, target_y)
    }
    
    /// Moves the mouse cursor relative to current position with animation
    pub fn move_by_with_duration(&mut self, dx: i32, dy: i32, duration: f64) -> MoverResult<()> {
        self.move_by_with_tween(dx, dy, duration, mover_core::linear_tween)
    }
    
    /// Moves the mouse cursor relative to current position with custom tweening
    pub fn move_by_with_tween(&mut self, dx: i32, dy: i32, duration: f64, tween: TweenFn) -> MoverResult<()> {
        let current_pos = self.position()?;
        let target_x = current_pos.x + dx;
        let target_y = current_pos.y + dy;
        self.move_to_with_tween(target_x, target_y, duration, tween)
    }
    
    // Click Functions
    // ===============
    
    /// Performs a mouse click at the current position
    /// 
    /// This function performs a mouse click with the specified button.
    /// If no button is specified, it defaults to the left button.
    /// 
    /// # Arguments
    /// 
    /// * `button` - The mouse button to click, or `None` for default (left)
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - Click performed successfully
    /// - `Err(MoverError)` - If the click failed
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_mouse::Mouse;
    /// use mover_core::types::MouseButton;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mouse = Mouse::new()?;
    ///     // Click with left button (default)
    ///     mouse.click(None)?;
    /// 
    ///     // Click with specific button
    ///     mouse.click(Some(MouseButton::Right))?;
    ///     Ok(())
    /// }
    /// ```
    pub fn click(&mut self, button: Option<MouseButton>) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        
        let enigo_button = self.convert_button(button)?;
        self.enigo.button(enigo_button, Direction::Click)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to click mouse: {}", e)
                )
            ))?;
        
        Ok(())
    }
    
    /// Performs a mouse click at specific coordinates
    pub fn click_at(&mut self, x: i32, y: i32, button: Option<MouseButton>) -> MoverResult<()> {
        self.move_to(x, y)?;
        self.click(button)
    }
    
    /// Performs a left mouse button click
    pub fn left_click(&mut self) -> MoverResult<()> {
        self.click(Some(MouseButton::Left))
    }
    
    /// Performs a right mouse button click
    pub fn right_click(&mut self) -> MoverResult<()> {
        self.click(Some(MouseButton::Right))
    }
    
    /// Performs a middle mouse button click
    pub fn middle_click(&mut self) -> MoverResult<()> {
        self.click(Some(MouseButton::Middle))
    }
    
    /// Performs a double click with the specified button
    pub fn double_click(&mut self, button: Option<MouseButton>) -> MoverResult<()> {
        self.click(button)?;
        thread::sleep(Duration::from_millis(50));
        self.click(button)
    }
    
    /// Performs a triple click with the specified button
    pub fn triple_click(&mut self, button: Option<MouseButton>) -> MoverResult<()> {
        self.click(button)?;
        thread::sleep(Duration::from_millis(50));
        self.click(button)?;
        thread::sleep(Duration::from_millis(50));
        self.click(button)
    }
    
    /// Presses down a mouse button
    pub fn mouse_down(&mut self, button: Option<MouseButton>) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        let enigo_button = self.convert_button(button)?;
        self.enigo.button(enigo_button, Direction::Press)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to press mouse button: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Releases a mouse button
    pub fn mouse_up(&mut self, button: Option<MouseButton>) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        let enigo_button = self.convert_button(button)?;
        self.enigo.button(enigo_button, Direction::Release)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to release mouse button: {}", e)
                )
            ))?;
        Ok(())
    }
    
    // Drag Functions
    // ===============
    
    /// Drags the mouse from current position to the specified coordinates
    pub fn drag_to(&mut self, x: i32, y: i32, button: Option<MouseButton>) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        self.mouse_down(Some(button))?;
        self.move_to(x, y)?;
        self.mouse_up(Some(button))
    }
    
    /// Drags the mouse relative to current position
    pub fn drag_by(&mut self, dx: i32, dy: i32, button: Option<MouseButton>) -> MoverResult<()> {
        let current_pos = self.position()?;
        let target_x = current_pos.x + dx;
        let target_y = current_pos.y + dy;
        self.drag_to(target_x, target_y, button)
    }
    
    // Scrolling Functions
    // ===================
    
    /// Performs vertical scrolling
    pub fn scroll(&mut self, clicks: i32) -> MoverResult<()> {
        self.enigo.scroll(clicks, Axis::Vertical)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to scroll: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Performs vertical scrolling (alias for scroll)
    pub fn vscroll(&mut self, clicks: i32) -> MoverResult<()> {
        self.scroll(clicks)
    }
    
    /// Performs horizontal scrolling
    pub fn hscroll(&mut self, clicks: i32) -> MoverResult<()> {
        self.enigo.scroll(clicks, Axis::Horizontal)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to scroll horizontally: {}", e)
                )
            ))?;
        Ok(())
    }
    
    // Utility Functions
    // =================
    
    /// Gets a point on a line between two points
    pub fn get_point_on_line(&self, start: Point, end: Point, ratio: f64) -> Point {
        let x = start.x + ((end.x - start.x) as f64 * ratio) as i32;
        let y = start.y + ((end.y - start.y) as f64 * ratio) as i32;
        Point::new(x, y)
    }
    
    /// Sleep for a given number of seconds
    pub fn sleep(&self, seconds: f64) {
        if seconds > 0.0 {
            thread::sleep(Duration::from_secs_f64(seconds));
        }
    }
    
    /// Countdown timer with visual feedback
    pub fn countdown(&self, seconds: u32) {
        for i in (1..=seconds).rev() {
            print!("{} ", i);
            std::io::stdout().flush().ok();
            thread::sleep(Duration::from_secs(1));
        }
        println!("0");
    }
    
    /// Convert MouseButton to enigo Button
    fn convert_button(&self, button: MouseButton) -> MoverResult<EnigoMouseButton> {
        match button {
            MouseButton::Left => Ok(EnigoMouseButton::Left),
            MouseButton::Right => Ok(EnigoMouseButton::Right),
            MouseButton::Middle => Ok(EnigoMouseButton::Middle),
            MouseButton::Primary => Ok(EnigoMouseButton::Left), // Default to left
            MouseButton::Secondary => Ok(EnigoMouseButton::Right), // Default to right
            MouseButton::Button4 => Ok(EnigoMouseButton::Left), // Fallback to left
            MouseButton::Button5 => Ok(EnigoMouseButton::Right), // Fallback to right
            MouseButton::Button6 => Ok(EnigoMouseButton::Left), // Fallback to left
            MouseButton::Button7 => Ok(EnigoMouseButton::Right), // Fallback to right
        }
    }
}

// No aliases - users should call Mouse::method() directly

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_creation() {
        let mouse = Mouse::new();
        assert!(mouse.is_ok());
    }

    #[test]
    fn test_screen_size() {
        let mouse = Mouse::new().unwrap();
        let size = mouse.size();
        assert!(size.is_ok());
        let size = size.unwrap();
        assert!(size.width > 0);
        assert!(size.height > 0);
    }

    #[test]
    fn test_position() {
        let mouse = Mouse::new().unwrap();
        let pos = mouse.position();
        assert!(pos.is_ok());
        let pos = pos.unwrap();
        assert!(pos.x >= 0);
        assert!(pos.y >= 0);
    }

    #[test]
    fn test_on_screen() {
        let mouse = Mouse::new().unwrap();
        let on_screen = mouse.on_screen(100, 100);
        assert!(on_screen.is_ok());
        assert!(on_screen.unwrap());
    }
}
