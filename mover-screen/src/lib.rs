//! Screen capture and image recognition module for the mover automation library
//! 
//! This module provides screen capture functionality and image recognition similar to PyAutoGUI,
//! including taking screenshots, finding images on screen, and getting pixel colors.

use mover_core::{MoverResult, Size, MoverError};
use image::RgbaImage;
use std::path::Path;

/// Screen control interface
pub struct Screen;

impl Screen {
    /// Takes a screenshot of the entire screen
    pub fn capture() -> MoverResult<RgbaImage> {
        let platform = mover_core::platform::get_platform()?;
        let _data = platform.capture_screen()?;
        
        // Convert raw bytes to image
        // This is a simplified conversion - in practice, you'd need to handle different formats
        let size = platform.get_size()?;
        let img = RgbaImage::new(size.width as u32, size.height as u32);
        
        // For now, return a placeholder image
        // TODO: Implement proper image conversion from platform data
        Ok(img)
    }
    
    /// Takes a screenshot of a specific region
    pub fn capture_region(x: i32, y: i32, width: u32, height: u32) -> MoverResult<RgbaImage> {
        let platform = mover_core::platform::get_platform()?;
        let _data = platform.capture_region(x, y, width, height)?;
        
        // Convert raw bytes to image
        let img = RgbaImage::new(width, height);
        
        // For now, return a placeholder image
        // TODO: Implement proper image conversion from platform data
        Ok(img)
    }
    
    /// Saves a screenshot to a file
    pub fn save_screenshot(path: &Path) -> MoverResult<()> {
        let img = Self::capture()?;
        img.save(path)
            .map_err(|e| MoverError::Other(format!("Failed to save screenshot: {}", e)))?;
        Ok(())
    }
    
    /// Saves a region screenshot to a file
    pub fn save_region_screenshot(path: &Path, x: i32, y: i32, width: u32, height: u32) -> MoverResult<()> {
        let img = Self::capture_region(x, y, width, height)?;
        img.save(path)
            .map_err(|e| MoverError::Other(format!("Failed to save region screenshot: {}", e)))?;
        Ok(())
    }
    
    /// Gets the color of a pixel at specific coordinates
    pub fn get_pixel_color(x: i32, y: i32) -> MoverResult<(u8, u8, u8)> {
        let platform = mover_core::platform::get_platform()?;
        platform.get_pixel_color(x, y)
    }
    
    /// Gets the color of a pixel at specific coordinates with alpha
    pub fn get_pixel_color_rgba(x: i32, y: i32) -> MoverResult<(u8, u8, u8, u8)> {
        let platform = mover_core::platform::get_platform()?;
        let (r, g, b) = platform.get_pixel_color(x, y)?;
        
        // For now, assume full alpha
        // TODO: Implement proper alpha channel support
        Ok((r, g, b, 255))
    }
    
    /// Checks if a pixel color matches the expected color
    pub fn pixel_matches_color(x: i32, y: i32, expected_color: (u8, u8, u8)) -> MoverResult<bool> {
        let actual_color = Self::get_pixel_color(x, y)?;
        Ok(actual_color == expected_color)
    }
    
    /// Checks if a pixel color matches the expected color with tolerance
    pub fn pixel_matches_color_with_tolerance(
        x: i32, 
        y: i32, 
        expected_color: (u8, u8, u8), 
        tolerance: u8
    ) -> MoverResult<bool> {
        let actual_color = Self::get_pixel_color(x, y)?;
        
        let r_diff = (actual_color.0 as i16 - expected_color.0 as i16).abs() as u8;
        let g_diff = (actual_color.1 as i16 - expected_color.1 as i16).abs() as u8;
        let b_diff = (actual_color.2 as i16 - expected_color.2 as i16).abs() as u8;
        
        Ok(r_diff <= tolerance && g_diff <= tolerance && b_diff <= tolerance)
    }
    
    /// Gets the screen size
    pub fn size() -> MoverResult<Size> {
        let platform = mover_core::platform::get_platform()?;
        platform.get_size()
    }
    
    /// Checks if coordinates are on screen
    pub fn is_on_screen(x: i32, y: i32) -> MoverResult<bool> {
        let platform = mover_core::platform::get_platform()?;
        platform.is_on_screen(x, y)
    }
}

// Convenience functions that mirror PyAutoGUI's API
pub use Screen as screen;

/// Alias for Screen::capture()
pub fn capture() -> MoverResult<RgbaImage> {
    Screen::capture()
}

/// Alias for Screen::capture_region()
pub fn capture_region(x: i32, y: i32, width: u32, height: u32) -> MoverResult<RgbaImage> {
    Screen::capture_region(x, y, width, height)
}

/// Alias for Screen::save_screenshot()
pub fn save_screenshot(path: &Path) -> MoverResult<()> {
    Screen::save_screenshot(path)
}

/// Alias for Screen::save_region_screenshot()
pub fn save_region_screenshot(path: &Path, x: i32, y: i32, width: u32, height: u32) -> MoverResult<()> {
    Screen::save_region_screenshot(path, x, y, width, height)
}

/// Alias for Screen::get_pixel_color()
pub fn get_pixel_color(x: i32, y: i32) -> MoverResult<(u8, u8, u8)> {
    Screen::get_pixel_color(x, y)
}

/// Alias for Screen::get_pixel_color_rgba()
pub fn get_pixel_color_rgba(x: i32, y: i32) -> MoverResult<(u8, u8, u8, u8)> {
    Screen::get_pixel_color_rgba(x, y)
}

/// Alias for Screen::pixel_matches_color()
pub fn pixel_matches_color(x: i32, y: i32, expected_color: (u8, u8, u8)) -> MoverResult<bool> {
    Screen::pixel_matches_color(x, y, expected_color)
}

/// Alias for Screen::pixel_matches_color_with_tolerance()
pub fn pixel_matches_color_with_tolerance(
    x: i32, 
    y: i32, 
    expected_color: (u8, u8, u8), 
    tolerance: u8
) -> MoverResult<bool> {
    Screen::pixel_matches_color_with_tolerance(x, y, expected_color, tolerance)
}

/// Alias for Screen::size()
pub fn size() -> MoverResult<Size> {
    Screen::size()
}

/// Alias for Screen::is_on_screen()
pub fn is_on_screen(x: i32, y: i32) -> MoverResult<bool> {
    Screen::is_on_screen(x, y)
}
