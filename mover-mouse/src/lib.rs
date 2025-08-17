//! Mouse control module for the mover automation library
//! 
//! This module provides comprehensive mouse control functionality similar to PyAutoGUI,
//! including movement, clicking, dragging, and scrolling operations.
//! 
//! Uses the `enigo` crate for cross-platform mouse control implementation.
//! 
//! # Features
//! 
//! - **Cross-Platform Support**: Works on Windows, macOS, and Linux
//! - **Real Mouse Control**: Actual mouse movement, clicking, and scrolling
//! - **Smooth Animations**: Tweening functions for natural mouse movements
//! - **Comprehensive API**: All the mouse functions you need for automation
//! - **Type Safety**: Rust's type system ensures safe mouse operations
//! 
//! # Quick Start
//! 
//! ```rust
//! use mover_mouse::*;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Get current mouse position
//!     let pos = position()?;
//!     println!("Mouse is at: {}", pos);
//!     
//!     // Move mouse to coordinates (100, 200)
//!     move_to(100, 200)?;
//!     
//!     // Click at current position
//!     left_click()?;
//!     
//!     Ok(())
//! }
//! ```

use mover_core::{MoverResult, Point, Size, TweenFn};
use enigo::{Button as EnigoMouseButton, Enigo, Settings, Direction, Coordinate, Axis, Mouse as EnigoMouse};
use std::time::Duration;
use std::thread;
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
///     // Get current position
///     let pos = Mouse::position()?;
/// 
///     // Move to specific coordinates
///     Mouse::move_to(100, 200)?;
/// 
///     // Click with left button
///     Mouse::left_click()?;
///     Ok(())
/// }
/// ```
pub struct Mouse;

impl Mouse {
    // Position and Information Functions
    // =================================
    
    /// Returns the current xy coordinates of the mouse cursor.
    pub fn position() -> MoverResult<Point> {
        let enigo = Enigo::new(&Settings::default()).unwrap();
        let (x, y) = enigo.location().unwrap();
        Ok(Point::new(x, y))
    }
    
    /// Returns the width and height of the primary display.
    pub fn size() -> MoverResult<Size> {
        let enigo = Enigo::new(&Settings::default()).unwrap();
        let (width, height) = enigo.main_display().unwrap();
        Ok(Size::new(width, height))
    }
    
    /// Returns whether the given xy coordinates are on the screen.
    pub fn on_screen(x: i32, y: i32) -> MoverResult<bool> {
        let screen_size = Self::size()?;
        Ok(x >= 0 && x < screen_size.width && y >= 0 && y < screen_size.height)
    }
    
    // Movement Functions
    // ==================
    
    /// Moves the mouse cursor to a point on the screen.
    pub fn move_to(x: i32, y: i32) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        enigo.move_mouse(x, y, Coordinate::Abs)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to move mouse: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Moves the mouse cursor to a point on the screen with linear animation.
    pub fn move_to_with_duration(x: i32, y: i32, duration: f64) -> MoverResult<()> {
        Self::move_to_with_tween(x, y, duration, mover_core::linear_tween)
    }
    
    /// Moves the mouse cursor to a point on the screen with custom tweening animation.
    pub fn move_to_with_tween(x: i32, y: i32, duration: f64, tween: TweenFn) -> MoverResult<()> {
        if duration <= 0.0 {
            return Self::move_to(x, y);
        }
        
        let start_pos = Self::position()?;
        let steps = (duration * 60.0).max(1.0) as usize; // 60 FPS
        
        for i in 0..=steps {
            let progress = i as f64 / steps as f64;
            let tweened_progress = tween(progress);
            
            let current_x = start_pos.x + ((x - start_pos.x) as f64 * tweened_progress) as i32;
            let current_y = start_pos.y + ((y - start_pos.y) as f64 * tweened_progress) as i32;
            
            Self::move_to(current_x, current_y)?;
            
            if i < steps {
                thread::sleep(Duration::from_secs_f64(duration / steps as f64));
            }
        }
        
        Ok(())
    }
    
    /// Moves the mouse cursor relative to current position
    pub fn move_by(dx: i32, dy: i32) -> MoverResult<()> {
        let current_pos = Self::position()?;
        let target_x = current_pos.x + dx;
        let target_y = current_pos.y + dy;
        Self::move_to(target_x, target_y)
    }
    
    /// Moves the mouse cursor relative to current position with animation
    pub fn move_by_with_duration(dx: i32, dy: i32, duration: f64) -> MoverResult<()> {
        Self::move_by_with_tween(dx, dy, duration, mover_core::linear_tween)
    }
    
    /// Moves the mouse cursor relative to current position with custom tweening
    pub fn move_by_with_tween(dx: i32, dy: i32, duration: f64, tween: TweenFn) -> MoverResult<()> {
        let current_pos = Self::position()?;
        let target_x = current_pos.x + dx;
        let target_y = current_pos.y + dy;
        Self::move_to_with_tween(target_x, target_y, duration, tween)
    }
    
    // Click Functions
    // ===============
    
    /// Performs a mouse click at the current position
    pub fn click(button: Option<MouseButton>) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        
        let enigo_button = Self::convert_button(button)?;
        enigo.button(enigo_button, Direction::Click)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to click mouse: {}", e)
                )
            ))?;
        
        Ok(())
    }
    
    /// Performs a mouse click at specific coordinates
    pub fn click_at(x: i32, y: i32, button: Option<MouseButton>) -> MoverResult<()> {
        Self::move_to(x, y)?;
        Self::click(button)
    }
    
    /// Performs a left mouse button click
    pub fn left_click() -> MoverResult<()> {
        Self::click(Some(MouseButton::Left))
    }
    
    /// Performs a left mouse button click at specific coordinates
    pub fn left_click_at(x: i32, y: i32) -> MoverResult<()> {
        Self::click_at(x, y, Some(MouseButton::Left))
    }
    
    /// Performs a right mouse button click
    pub fn right_click() -> MoverResult<()> {
        Self::click(Some(MouseButton::Right))
    }
    
    /// Performs a right mouse button click at specific coordinates
    pub fn right_click_at(x: i32, y: i32) -> MoverResult<()> {
        Self::click_at(x, y, Some(MouseButton::Right))
    }
    
    /// Performs a middle mouse button click
    pub fn middle_click() -> MoverResult<()> {
        Self::click(Some(MouseButton::Middle))
    }
    
    /// Performs a middle mouse button click at specific coordinates
    pub fn middle_click_at(x: i32, y: i32) -> MoverResult<()> {
        Self::click_at(x, y, Some(MouseButton::Middle))
    }
    
    /// Performs a double click
    pub fn double_click(button: Option<MouseButton>) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        
        // Enigo doesn't have double click, so we'll simulate it
        for _ in 0..2 {
            Self::click(Some(button))?;
            thread::sleep(Duration::from_millis(50)); // Small delay between clicks
        }
        
        Ok(())
    }
    
    /// Performs a double click at specific coordinates
    pub fn double_click_at(x: i32, y: i32, button: Option<MouseButton>) -> MoverResult<()> {
        Self::move_to(x, y)?;
        Self::double_click(button)
    }
    
    /// Performs a triple click
    pub fn triple_click(button: Option<MouseButton>) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        
        // Enigo doesn't have triple click, so we'll simulate it
        for _ in 0..3 {
            Self::click(Some(button))?;
            thread::sleep(Duration::from_millis(50)); // Small delay between clicks
        }
        
        Ok(())
    }
    
    /// Performs a triple click at specific coordinates
    pub fn triple_click_at(x: i32, y: i32, button: Option<MouseButton>) -> MoverResult<()> {
        Self::move_to(x, y)?;
        Self::triple_click(button)
    }
    
    // Button State Control
    // ====================
    
    /// Presses a mouse button down (but not up)
    pub fn mouse_down(button: Option<MouseButton>) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        
        let enigo_button = Self::convert_button(button)?;
        enigo.button(enigo_button, Direction::Press)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to press mouse button: {}", e)
                )
            ))?;
        
        Ok(())
    }
    
    /// Releases a mouse button up (but not down beforehand)
    pub fn mouse_up(button: Option<MouseButton>) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        
        let enigo_button = Self::convert_button(button)?;
        enigo.button(enigo_button, Direction::Release)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to release mouse button: {}", e)
                )
            ))?;
        
        Ok(())
    }
    
    // Drag Functions
    // ==============
    
    /// Performs a mouse drag to absolute coordinates
    pub fn drag_to(x: i32, y: i32, button: Option<MouseButton>) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        
        // Press the button
        Self::mouse_down(Some(button))?;
        
        // Move to destination
        Self::move_to(x, y)?;
        
        // Release the button
        Self::mouse_up(Some(button))?;
        
        Ok(())
    }
    
    /// Performs a mouse drag to absolute coordinates with animation
    pub fn drag_to_with_duration(x: i32, y: i32, button: Option<MouseButton>, duration: f64) -> MoverResult<()> {
        Self::drag_to_with_tween(x, y, button, duration, mover_core::linear_tween)
    }
    
    /// Performs a mouse drag to absolute coordinates with custom tweening
    pub fn drag_to_with_tween(x: i32, y: i32, button: Option<MouseButton>, duration: f64, tween: TweenFn) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        
        // Press the button
        Self::mouse_down(Some(button))?;
        
        // Move with tweening
        Self::move_to_with_tween(x, y, duration, tween)?;
        
        // Release the button
        Self::mouse_up(Some(button))?;
        
        Ok(())
    }
    
    /// Performs a mouse drag relative to current position
    pub fn drag_by(dx: i32, dy: i32, button: Option<MouseButton>) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        
        // Press the button
        Self::mouse_down(Some(button))?;
        
        // Move relative
        Self::move_by(dx, dy)?;
        
        // Release the button
        Self::mouse_up(Some(button))?;
        
        Ok(())
    }
    
    /// Performs a mouse drag relative to current position with animation
    pub fn drag_by_with_duration(dx: i32, dy: i32, button: Option<MouseButton>, duration: f64) -> MoverResult<()> {
        Self::drag_by_with_tween(dx, dy, button, duration, mover_core::linear_tween)
    }
    
    /// Performs a mouse drag relative to current position with custom tweening
    pub fn drag_by_with_tween(dx: i32, dy: i32, button: Option<MouseButton>, duration: f64, tween: TweenFn) -> MoverResult<()> {
        let button = button.unwrap_or_default();
        
        // Press the button
        Self::mouse_down(Some(button))?;
        
        // Move with tweening
        Self::move_by_with_tween(dx, dy, duration, tween)?;
        
        // Release the button
        Self::mouse_up(Some(button))?;
        
        Ok(())
    }
    
    // Scroll Functions
    // ================
    
    /// Performs a scroll of the mouse scroll wheel
    pub fn scroll(clicks: i32) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        enigo.scroll(clicks, Axis::Vertical)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to scroll: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Performs a vertical scroll
    pub fn vscroll(clicks: i32) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        enigo.scroll(clicks, Axis::Vertical)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to scroll vertically: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Performs a horizontal scroll
    pub fn hscroll(clicks: i32) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        enigo.scroll(clicks, Axis::Horizontal)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to scroll horizontally: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Performs a scroll at specific coordinates
    pub fn scroll_at(clicks: i32, x: i32, y: i32) -> MoverResult<()> {
        Self::move_to(x, y)?;
        Self::scroll(clicks)
    }
    
    /// Performs a vertical scroll at specific coordinates
    pub fn vscroll_at(clicks: i32, x: i32, y: i32) -> MoverResult<()> {
        Self::move_to(x, y)?;
        Self::vscroll(clicks)
    }
    
    /// Performs a horizontal scroll at specific coordinates
    pub fn hscroll_at(clicks: i32, x: i32, y: i32) -> MoverResult<()> {
        Self::move_to(x, y)?;
        Self::hscroll(clicks)
    }
    
    // Utility Functions
    // =================
    
    /// Gets a point on a line between two points at a given proportion
    pub fn get_point_on_line(x1: i32, y1: i32, x2: i32, y2: i32, n: f64) -> Point {
        let x = ((x2 - x1) as f64 * n) + x1 as f64;
        let y = ((y2 - y1) as f64 * n) + y1 as f64;
        Point::new(x as i32, y as i32)
    }
    
    /// Sleep for a given number of seconds
    pub fn sleep(seconds: f64) {
        if seconds > 0.0 {
            thread::sleep(Duration::from_secs_f64(seconds));
        }
    }
    
    /// Countdown timer
    pub fn countdown(seconds: u32) {
        for i in (1..=seconds).rev() {
            print!("{} ", i);
            std::io::stdout().flush().ok();
            thread::sleep(Duration::from_secs(1));
        }
        println!("0");
    }
    
    /// Converts mover MouseButton to enigo MouseButton.
    fn convert_button(button: MouseButton) -> MoverResult<EnigoMouseButton> {
        match button {
            MouseButton::Left => Ok(EnigoMouseButton::Left),
            MouseButton::Right => Ok(EnigoMouseButton::Right),
            MouseButton::Middle => Ok(EnigoMouseButton::Middle),
            MouseButton::Primary => Ok(EnigoMouseButton::Left), // Default to left
            MouseButton::Secondary => Ok(EnigoMouseButton::Right), // Default to right
            MouseButton::Button4 => Ok(EnigoMouseButton::Left), // Enigo doesn't support Button4, fallback to Left
            MouseButton::Button5 => Ok(EnigoMouseButton::Right), // Enigo doesn't support Button5, fallback to Right
            MouseButton::Button6 => Ok(EnigoMouseButton::Left), // Enigo doesn't support Button6, fallback to Left
            MouseButton::Button7 => Ok(EnigoMouseButton::Right), // Enigo doesn't support Button7, fallback to Right
        }
    }
}

// Convenience functions that mirror PyAutoGUI's API
pub use Mouse as mouse;

/// Alias for Mouse::position()
pub fn position() -> MoverResult<Point> {
    Mouse::position()
}

/// Alias for Mouse::size()
pub fn size() -> MoverResult<Size> {
    Mouse::size()
}

/// Alias for Mouse::on_screen()
pub fn on_screen(x: i32, y: i32) -> MoverResult<bool> {
    Mouse::on_screen(x, y)
}

/// Alias for Mouse::move_to()
pub fn move_to(x: i32, y: i32) -> MoverResult<()> {
    Mouse::move_to(x, y)
}

/// Alias for Mouse::move_by()
pub fn move_by(dx: i32, dy: i32) -> MoverResult<()> {
    Mouse::move_by(dx, dy)
}

/// Alias for Mouse::click()
pub fn click(button: Option<MouseButton>) -> MoverResult<()> {
    Mouse::click(button)
}

/// Alias for Mouse::left_click()
pub fn left_click() -> MoverResult<()> {
    Mouse::left_click()
}

/// Alias for Mouse::right_click()
pub fn right_click() -> MoverResult<()> {
    Mouse::right_click()
}

/// Alias for Mouse::middle_click()
pub fn middle_click() -> MoverResult<()> {
    Mouse::middle_click()
}

/// Alias for Mouse::double_click()
pub fn double_click(button: Option<MouseButton>) -> MoverResult<()> {
    Mouse::double_click(button)
}

/// Alias for Mouse::triple_click()
pub fn triple_click(button: Option<MouseButton>) -> MoverResult<()> {
    Mouse::triple_click(button)
}

/// Alias for Mouse::drag_to()
pub fn drag_to(x: i32, y: i32, button: Option<MouseButton>) -> MoverResult<()> {
    Mouse::drag_to(x, y, button)
}

/// Alias for Mouse::drag_by()
pub fn drag_by(dx: i32, dy: i32, button: Option<MouseButton>) -> MoverResult<()> {
    Mouse::drag_by(dx, dy, button)
}

/// Alias for Mouse::scroll()
pub fn scroll(clicks: i32) -> MoverResult<()> {
    Mouse::scroll(clicks)
}

/// Alias for Mouse::vscroll()
pub fn vscroll(clicks: i32) -> MoverResult<()> {
    Mouse::vscroll(clicks)
}

/// Alias for Mouse::hscroll()
pub fn hscroll(clicks: i32) -> MoverResult<()> {
    Mouse::hscroll(clicks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_button_conversion() {
        assert!(Mouse::convert_button(MouseButton::Left).is_ok());
        assert!(Mouse::convert_button(MouseButton::Right).is_ok());
        assert!(Mouse::convert_button(MouseButton::Middle).is_ok());
        assert!(Mouse::convert_button(MouseButton::Primary).is_ok());
        assert!(Mouse::convert_button(MouseButton::Secondary).is_ok());
    }

    #[test]
    fn test_point_on_line() {
        let point = Mouse::get_point_on_line(0, 0, 100, 100, 0.5);
        assert_eq!(point, Point::new(50, 50));
    }

    #[test]
    fn test_on_screen() {
        // This test might fail if run on a very small screen
        let result = Mouse::on_screen(100, 100);
        assert!(result.is_ok());
    }
}
