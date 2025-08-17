//! Keyboard control module for the mover automation library
//! 
//! This module provides comprehensive keyboard control functionality similar to PyAutoGUI,
//! including typing, key presses, and hotkey combinations.
//! 
//! Uses the `enigo` crate for cross-platform keyboard control implementation.
//! 
//! # Features
//! 
//! - **Cross-Platform Support**: Works on Windows, macOS, and Linux
//! - **Real Keyboard Control**: Actual key presses, releases, and typing
//! - **Hotkey Support**: Complex key combinations and shortcuts
//! - **Comprehensive API**: All the keyboard functions you need for automation
//! - **Type Safety**: Rust's type system ensures safe keyboard operations
//! - **Error Handling**: Proper error handling with `Result` types
//! 
//! # Quick Start
//! 
//! ```rust
//! use mover_keyboard::*;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Type some text
//!     type_string("Hello, World!")?;
//!     
//!     // Press a key
//!     press_key("enter")?;
//!     
//!     // Press a hotkey combination
//!     press_hotkey(&["ctrl", "c"])?;
//!     
//!     Ok(())
//! }
//! ```

use mover_core::MoverResult;
use std::time::Duration;
use std::thread;
use enigo::{Enigo, Settings, Direction, Keyboard as EnigoKeyboard};

/// Keyboard control interface
pub struct Keyboard;

impl Keyboard {
    /// Types a string of text
    pub fn type_string(text: &str) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to create Enigo instance: {}", e)
                )
            ))?;
        
        // Use the text method for faster typing
        enigo.text(text)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to type text: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Types a string of text with a delay between characters
    pub fn type_string_with_interval(text: &str, interval: f64) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to create Enigo instance: {}", e)
                )
            ))?;
        
        // For interval typing, we need to type character by character
        for c in text.chars() {
            enigo.key(enigo::Key::Unicode(c), Direction::Click)
                .map_err(|e| mover_core::MoverError::PlatformError(
                    mover_core::PlatformError::UnsupportedOperation(
                        format!("Failed to type character '{}': {}", c, e)
                    )
                ))?;
            if interval > 0.0 {
                thread::sleep(Duration::from_secs_f64(interval));
            }
        }
        
        Ok(())
    }
    
    /// Presses a key
    pub fn press_key(key: &str) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to create Enigo instance: {}", e)
                )
            ))?;
        let key_code = convert_key(key)?;
        enigo.key(key_code, Direction::Press)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to press key: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Releases a key
    pub fn release_key(key: &str) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to create Enigo instance: {}", e)
                )
            ))?;
        let key_code = convert_key(key)?;
        enigo.key(key_code, Direction::Release)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to release key: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Presses and releases a key (tap)
    pub fn tap_key(key: &str) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to create Enigo instance: {}", e)
                )
            ))?;
        let key_code = convert_key(key)?;
        enigo.key(key_code, Direction::Click)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to tap key: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Presses multiple keys in sequence
    pub fn press_keys(keys: &[&str]) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to create Enigo instance: {}", e)
                )
            ))?;
        
        for key in keys {
            let key_code = convert_key(key)?;
            enigo.key(key_code, Direction::Click)
                .map_err(|e| mover_core::MoverError::PlatformError(
                    mover_core::PlatformError::UnsupportedOperation(
                        format!("Failed to press key '{}': {}", key, e)
                    )
                ))?;
            thread::sleep(Duration::from_millis(50)); // Small delay between keys
        }
        
        Ok(())
    }
    
    /// Presses a hotkey combination (all keys down, then all keys up)
    pub fn press_hotkey(keys: &[&str]) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to create Enigo instance: {}", e)
                )
            ))?;
        
        // Press all keys down
        for key in keys {
            let key_code = convert_key(key)?;
            enigo.key(key_code, Direction::Press)
                .map_err(|e| mover_core::MoverError::PlatformError(
                    mover_core::PlatformError::UnsupportedOperation(
                        format!("Failed to press key '{}': {}", key, e)
                    )
                ))?;
        }
        
        // Small delay to ensure all keys are pressed
        thread::sleep(Duration::from_millis(50));
        
        // Release all keys up (in reverse order for some combinations)
        for key in keys.iter().rev() {
            let key_code = convert_key(key)?;
            enigo.key(key_code, Direction::Release)
                .map_err(|e| mover_core::MoverError::PlatformError(
                    mover_core::PlatformError::UnsupportedOperation(
                        format!("Failed to release key '{}': {}", key, e)
                    )
                ))?;
        }
        
        Ok(())
    }
    
    /// Presses a key multiple times
    pub fn press_key_multiple(key: &str, times: u32, interval: f64) -> MoverResult<()> {
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to create Enigo instance: {}", e)
                )
            ))?;
        let key_code = convert_key(key)?;
        
        for _ in 0..times {
            enigo.key(key_code, Direction::Click)
                .map_err(|e| mover_core::MoverError::PlatformError(
                    mover_core::PlatformError::UnsupportedOperation(
                        format!("Failed to press key: {}", e)
                    )
                ))?;
            if interval > 0.0 {
                thread::sleep(Duration::from_secs_f64(interval));
            }
        }
        
        Ok(())
    }
    
    /// Sleep for a given number of seconds
    pub fn sleep(seconds: f64) {
        if seconds > 0.0 {
            thread::sleep(Duration::from_secs_f64(seconds));
        }
    }
}

/// Convert string key names to enigo Key codes
fn convert_key(key: &str) -> MoverResult<enigo::Key> {
    let key_lower = key.to_lowercase();
    
    match key_lower.as_str() {
        // Basic keys
        "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" |
        "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" => {
            Ok(enigo::Key::Unicode(key_lower.chars().next().unwrap()))
        }
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            Ok(enigo::Key::Unicode(key_lower.chars().next().unwrap()))
        }
        
        // Special keys
        "enter" | "return" => Ok(enigo::Key::Return),
        "space" => Ok(enigo::Key::Space),
        "tab" => Ok(enigo::Key::Tab),
        "escape" | "esc" => Ok(enigo::Key::Escape),
        "backspace" => Ok(enigo::Key::Backspace),
        "delete" | "del" => Ok(enigo::Key::Delete),
        "home" => Ok(enigo::Key::Home),
        "end" => Ok(enigo::Key::End),
        "pageup" | "pgup" => Ok(enigo::Key::PageUp),
        "pagedown" | "pgdn" => Ok(enigo::Key::PageDown),
        
        // Arrow keys
        "up" => Ok(enigo::Key::UpArrow),
        "down" => Ok(enigo::Key::DownArrow),
        "left" => Ok(enigo::Key::LeftArrow),
        "right" => Ok(enigo::Key::RightArrow),
        
        // Function keys
        "f1" => Ok(enigo::Key::F1),
        "f2" => Ok(enigo::Key::F2),
        "f3" => Ok(enigo::Key::F3),
        "f4" => Ok(enigo::Key::F4),
        "f5" => Ok(enigo::Key::F5),
        "f6" => Ok(enigo::Key::F6),
        "f7" => Ok(enigo::Key::F7),
        "f8" => Ok(enigo::Key::F8),
        "f9" => Ok(enigo::Key::F9),
        "f10" => Ok(enigo::Key::F10),
        "f11" => Ok(enigo::Key::F11),
        "f12" => Ok(enigo::Key::F12),
        
        // Modifier keys
        "ctrl" | "control" => Ok(enigo::Key::Control),
        "alt" => Ok(enigo::Key::Alt),
        "shift" => Ok(enigo::Key::Shift),
        "meta" | "win" | "command" => Ok(enigo::Key::Meta),
        
        // Punctuation and symbols
        "!" | "@" | "#" | "$" | "%" | "^" | "&" | "*" | "(" | ")" | "-" | "_" | "=" | "+" |
        "[" | "]" | "{" | "}" | "\\" | "|" | ";" | ":" | "'" | "\"" | "," | "." | "/" | "?" |
        "`" | "~" => {
            Ok(enigo::Key::Unicode(key_lower.chars().next().unwrap()))
        }
        
        _ => Err(mover_core::MoverError::PlatformError(
            mover_core::PlatformError::UnsupportedOperation(
                format!("Unsupported key: {}", key)
            )
        ))
    }
}

// Convenience functions that mirror PyAutoGUI's API
pub use Keyboard as keyboard;

/// Alias for Keyboard::type_string()
pub fn type_string(text: &str) -> MoverResult<()> {
    Keyboard::type_string(text)
}

/// Alias for Keyboard::type_string_with_interval()
pub fn type_string_with_interval(text: &str, interval: f64) -> MoverResult<()> {
    Keyboard::type_string_with_interval(text, interval)
}

/// Alias for Keyboard::press_key()
pub fn press_key(key: &str) -> MoverResult<()> {
    Keyboard::press_key(key)
}

/// Alias for Keyboard::release_key()
pub fn release_key(key: &str) -> MoverResult<()> {
    Keyboard::release_key(key)
}

/// Alias for Keyboard::tap_key()
pub fn tap_key(key: &str) -> MoverResult<()> {
    Keyboard::tap_key(key)
}

/// Alias for Keyboard::press_keys()
pub fn press_keys(keys: &[&str]) -> MoverResult<()> {
    Keyboard::press_keys(keys)
}

/// Alias for Keyboard::press_hotkey()
pub fn press_hotkey(keys: &[&str]) -> MoverResult<()> {
    Keyboard::press_hotkey(keys)
}

/// Alias for Keyboard::press_key_multiple()
pub fn press_key_multiple(key: &str, times: u32, interval: f64) -> MoverResult<()> {
    Keyboard::press_key_multiple(key, times, interval)
}

/// Alias for Keyboard::sleep()
pub fn sleep(seconds: f64) {
    Keyboard::sleep(seconds)
}
