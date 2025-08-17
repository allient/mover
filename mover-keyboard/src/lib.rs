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
//! use mover_keyboard::Keyboard;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a keyboard instance
//!     let mut keyboard = Keyboard::new()?;
//!     
//!     // Type some text
//!     keyboard.type_string("Hello, World!")?;
//!     
//!     // Press a key
//!     keyboard.press_key("enter")?;
//!     
//!     // Press a hotkey combination
//!     keyboard.press_hotkey(&["ctrl", "c"])?;
//!     
//!     Ok(())
//! }
//! ```
//! 
//! # Examples
//! 
//! ## Basic Typing
//! 
//! ```rust
//! use mover_keyboard::Keyboard;
//! 
//! fn basic_typing() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut keyboard = Keyboard::new()?;
//!     
//!     // Type text instantly
//!     keyboard.type_string("Hello, World!")?;
//!     
//!     // Type with delays between characters
//!     keyboard.type_string_with_interval("Slow typing...", 0.5)?;
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Hotkey Combinations
//! 
//! ```rust
//! use mover_keyboard::Keyboard;
//! 
//! fn hotkey_examples() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut keyboard = Keyboard::new()?;
//!     
//!     // Common shortcuts
//!     keyboard.press_hotkey(&["ctrl", "c"])?;    // Copy
//!     keyboard.press_hotkey(&["ctrl", "v"])?;    // Paste
//!     keyboard.press_hotkey(&["ctrl", "z"])?;    // Undo
//!     
//!     Ok(())
//! }
//! ```

use mover_core::MoverResult;
use std::time::Duration;
use std::thread;
use enigo::{Enigo, Settings, Direction, Keyboard as EnigoKeyboard};

/// Keyboard control interface providing comprehensive automation capabilities.
/// 
/// This struct contains all the methods needed for keyboard control, including:
/// - Text typing (instant and with intervals)
/// - Individual key operations (press, release, tap)
/// - Hotkey combinations and shortcuts
/// - Utility functions for automation
/// 
/// # Examples
/// 
/// ```rust
/// use mover_keyboard::Keyboard;
/// 
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a keyboard instance
///     let mut keyboard = Keyboard::new()?;
///     
///     // Type some text
///     keyboard.type_string("Hello, World!")?;
///     
///     // Press Enter
///     keyboard.press_key("enter")?;
///     
///     // Use a hotkey combination
///     keyboard.press_hotkey(&["ctrl", "c"])?;
///     
///     Ok(())
/// }
/// ```
/// 
/// # Platform Support
/// 
/// - **Windows**: Full support for all features
/// - **macOS**: Full support, may require accessibility permissions
/// - **Linux**: Full support, may require X11 or Wayland setup
pub struct Keyboard {
    enigo: Enigo,
}

impl Keyboard {
    /// Create a new Keyboard instance.
    /// 
    /// This method initializes the underlying `enigo` instance for cross-platform
    /// keyboard control. The instance must be mutable (`mut`) because most keyboard
    /// operations modify internal state.
    /// 
    /// # Returns
    /// 
    /// - `Ok(Keyboard)` - A new keyboard instance ready for use
    /// - `Err(Box<dyn std::error::Error>)` - If initialization fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_keyboard::Keyboard;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut keyboard = Keyboard::new()?;
    ///     println!("Keyboard instance created successfully!");
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// - The underlying `enigo` instance cannot be created
    /// - Platform-specific keyboard access is not available
    /// - Required permissions are not granted
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let enigo = Enigo::new(&Settings::default())
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to create Enigo instance: {}", e)
                )
            ))?;
        Ok(Keyboard { enigo })
    }

    /// Types a string of text instantly.
    /// 
    /// This method uses `enigo`'s optimized text method for fast typing.
    /// The text is typed as quickly as possible without delays between characters.
    /// 
    /// # Arguments
    /// 
    /// * `text` - The string to type
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - Text typed successfully
    /// - `Err(MoverError)` - If typing fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_keyboard::Keyboard;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut keyboard = Keyboard::new()?;
    ///     
    ///     // Type a simple message
    ///     keyboard.type_string("Hello, World!")?;
    ///     
    ///     // Type with special characters
    ///     keyboard.type_string("Special chars: !@#$%^&*()")?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Performance
    /// 
    /// This method is optimized for speed and will type text as fast as possible.
    /// For slower, more human-like typing, use `type_string_with_interval`.
    pub fn type_string(&mut self, text: &str) -> MoverResult<()> {
        // Use the text method for faster typing
        self.enigo.text(text)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to type text: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Types a string of text with a delay between characters.
    /// 
    /// This method types text character by character with the specified interval
    /// between each character. Useful for creating more human-like typing patterns
    /// or when you need to control the timing of text input.
    /// 
    /// # Arguments
    /// 
    /// * `text` - The string to type
    /// * `interval` - Delay between characters in seconds (can be fractional)
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - Text typed successfully
    /// - `Err(MoverError)` - If typing fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_keyboard::Keyboard;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut keyboard = Keyboard::new()?;
    ///     
    ///     // Type slowly for human-like behavior
    ///     keyboard.type_string_with_interval("Slow typing...", 0.5)?;
    ///     
    ///     // Type very slowly for emphasis
    ///     keyboard.type_string_with_interval("IMPORTANT", 1.0)?;
    ///     
    ///     // Type quickly but with small delays
    ///     keyboard.type_string_with_interval("Quick with delays", 0.1)?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Performance Notes
    /// 
    /// - This method is slower than `type_string` due to individual character processing
    /// - Use `interval = 0.0` for fastest possible typing (same as `type_string`)
    /// - Larger intervals create more human-like typing patterns
    pub fn type_string_with_interval(&mut self, text: &str, interval: f64) -> MoverResult<()> {
        // For interval typing, we need to type character by character
        for c in text.chars() {
            self.enigo.key(enigo::Key::Unicode(c), Direction::Click)
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
    
    /// Presses a key down (but does not release it).
    /// 
    /// This method is useful for holding down modifier keys or creating
    /// custom key combinations. Remember to call `release_key` to release
    /// the pressed key.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to press (e.g., "shift", "ctrl", "a")
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - Key pressed successfully
    /// - `Err(MoverError)` - If key press fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_keyboard::Keyboard;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut keyboard = Keyboard::new()?;
    ///     
    ///     // Hold down Shift
    ///     keyboard.press_key("shift")?;
    ///     
    ///     // Type a capital letter
    ///     keyboard.type_string("A")?;
    ///     
    ///     // Release Shift
    ///     keyboard.release_key("shift")?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Important Notes
    /// 
    /// - Always pair `press_key` with `release_key` to avoid stuck keys
    /// - Some applications may not respond well to held-down keys
    /// - Use `tap_key` for simple press-and-release operations
    pub fn press_key(&mut self, key: &str) -> MoverResult<()> {
        let key_code = convert_key(key)?;
        self.enigo.key(key_code, Direction::Press)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to press key: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Releases a previously pressed key.
    /// 
    /// This method releases a key that was previously pressed with `press_key`.
    /// It's essential to call this after `press_key` to avoid stuck keys.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to release (must match the key used in `press_key`)
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - Key released successfully
    /// - `Err(MoverError)` - If key release fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_keyboard::Keyboard;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut keyboard = Keyboard::new()?;
    ///     
    ///     // Press and hold Ctrl
    ///     keyboard.press_key("ctrl")?;
    ///     
    ///     // Press A while Ctrl is held
    ///     keyboard.press_key("a")?;
    ///     keyboard.release_key("a")?;
    ///     
    ///     // Release Ctrl
    ///     keyboard.release_key("ctrl")?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Safety
    /// 
    /// - Always release keys in the reverse order they were pressed
    /// - Unreleased keys can cause system-wide issues
    /// - Consider using `press_hotkey` for common combinations
    pub fn release_key(&mut self, key: &str) -> MoverResult<()> {
        let key_code = convert_key(key)?;
        self.enigo.key(key_code, Direction::Release)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to release key: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Presses and immediately releases a key (tap).
    /// 
    /// This is the most common keyboard operation - a quick press and release
    /// of a single key. Equivalent to calling `press_key` followed by `release_key`.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to tap (e.g., "enter", "space", "a")
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - Key tapped successfully
    /// - `Err(MoverError)` - If key tap fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_keyboard::Keyboard;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut keyboard = Keyboard::new()?;
    ///     
    ///     // Tap Enter to submit
    ///     keyboard.tap_key("enter")?;
    ///     
    ///     // Tap Space to add space
    ///     keyboard.tap_key("space")?;
    ///     
    ///     // Tap Tab to move to next field
    ///     keyboard.tap_key("tab")?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Common Use Cases
    /// 
    /// - Navigation keys: `"up"`, `"down"`, `"left"`, `"right"`
    /// - Function keys: `"f1"`, `"f2"`, `"f3"`, etc.
    /// - Special keys: `"enter"`, `"space"`, `"tab"`, `"escape"`
    pub fn tap_key(&mut self, key: &str) -> MoverResult<()> {
        let key_code = convert_key(key)?;
        self.enigo.key(key_code, Direction::Click)
            .map_err(|e| mover_core::MoverError::PlatformError(
                mover_core::PlatformError::UnsupportedOperation(
                    format!("Failed to tap key: {}", e)
                )
            ))?;
        Ok(())
    }
    
    /// Presses multiple keys in sequence.
    /// 
    /// This method presses and releases each key in the provided array,
    /// with a small delay between each key. Useful for typing words
    /// or executing simple sequences.
    /// 
    /// # Arguments
    /// 
    /// * `keys` - Array of keys to press in sequence
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - All keys pressed successfully
    /// - `Err(MoverError)` - If any key press fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_keyboard::Keyboard;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut keyboard = Keyboard::new()?;
    ///     
    ///     // Type a word character by character
    ///     keyboard.press_keys(&["h", "e", "l", "l", "o"])?;
    ///     
    ///     // Execute a simple sequence
    ///     keyboard.press_keys(&["tab", "space", "enter"])?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Performance Notes
    /// 
    /// - Each key is pressed and released individually
    /// - There's a 50ms delay between keys for reliability
    /// - For faster typing, use `type_string` instead
    pub fn press_keys(&mut self, keys: &[&str]) -> MoverResult<()> {
        for key in keys {
            let key_code = convert_key(key)?;
            self.enigo.key(key_code, Direction::Click)
                .map_err(|e| mover_core::MoverError::PlatformError(
                    mover_core::PlatformError::UnsupportedOperation(
                        format!("Failed to press key '{}': {}", key, e)
                    )
                ))?;
            thread::sleep(Duration::from_millis(50)); // Small delay between keys
        }
        
        Ok(())
    }
    
    /// Presses a hotkey combination (all keys down, then all keys up).
    /// 
    /// This method is essential for common keyboard shortcuts like Ctrl+C, Ctrl+V,
    /// Alt+Tab, etc. It presses all keys down simultaneously, then releases them
    /// in reverse order for maximum compatibility.
    /// 
    /// # Arguments
    /// 
    /// * `keys` - Array of keys to press simultaneously
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - Hotkey combination executed successfully
    /// - `Err(MoverError)` - If any key operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_keyboard::Keyboard;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut keyboard = Keyboard::new()?;
    ///     
    ///     // Common shortcuts
    ///     keyboard.press_hotkey(&["ctrl", "c"])?;    // Copy
    ///     keyboard.press_hotkey(&["ctrl", "v"])?;    // Paste
    ///     keyboard.press_hotkey(&["ctrl", "z"])?;    // Undo
    ///     keyboard.press_hotkey(&["ctrl", "a"])?;    // Select All
    ///     
    ///     // Complex combinations
    ///     keyboard.press_hotkey(&["ctrl", "shift", "a"])?;  // Select All (alternative)
    ///     keyboard.press_hotkey(&["alt", "tab"])?;          // Switch windows
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Common Hotkey Combinations
    /// 
    /// | Combination | Action |
    /// |-------------|---------|
    /// | `["ctrl", "c"]` | Copy |
    /// | `["ctrl", "v"]` | Paste |
    /// | `["ctrl", "x"]` | Cut |
    /// | `["ctrl", "z"]` | Undo |
    /// | `["ctrl", "a"]` | Select All |
    /// | `["alt", "tab"]` | Switch Windows |
    /// | `["ctrl", "shift", "esc"]` | Task Manager |
    /// 
    /// # Technical Details
    /// 
    /// - All keys are pressed down simultaneously
    /// - A 50ms delay ensures all keys are registered
    /// - Keys are released in reverse order for compatibility
    /// - This method handles the complexity of proper hotkey execution
    pub fn press_hotkey(&mut self, keys: &[&str]) -> MoverResult<()> {
        // Press all keys down
        for key in keys {
            let key_code = convert_key(key)?;
            self.enigo.key(key_code, Direction::Press)
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
            self.enigo.key(key_code, Direction::Release)
                .map_err(|e| mover_core::MoverError::PlatformError(
                    mover_core::PlatformError::UnsupportedOperation(
                        format!("Failed to release key '{}': {}", key, e)
                    )
                ))?;
        }
        
        Ok(())
    }
    
    /// Presses a key multiple times with specified intervals.
    /// 
    /// This method is useful for repeating actions like pressing arrow keys
    /// multiple times, or creating patterns with delays.
    /// 
    /// # Arguments
    /// 
    /// * `key` - The key to press multiple times
    /// * `times` - Number of times to press the key
    /// * `interval` - Delay between presses in seconds (can be fractional)
    /// 
    /// # Returns
    /// 
    /// - `Ok(())` - All key presses completed successfully
    /// - `Err(MoverError)` - If any key press fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_keyboard::Keyboard;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut keyboard = Keyboard::new()?;
    ///     
    ///     // Press 'a' 5 times with 0.2 second intervals
    ///     keyboard.press_key_multiple("a", 5, 0.2)?;
    ///     
    ///     // Press arrow key 10 times quickly
    ///     keyboard.press_key_multiple("right", 10, 0.1)?;
    ///     
    ///     // Press space 3 times slowly
    ///     keyboard.press_key_multiple("space", 3, 1.0)?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Use Cases
    /// 
    /// - **Navigation**: Move cursor multiple positions
    /// - **Repeated Actions**: Type the same character multiple times
    /// - **Patterns**: Create rhythmic keyboard patterns
    /// - **Testing**: Verify repeated key functionality
    pub fn press_key_multiple(&mut self, key: &str, times: u32, interval: f64) -> MoverResult<()> {
        let key_code = convert_key(key)?;
        
        for _ in 0..times {
            self.enigo.key(key_code, Direction::Click)
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
    
    /// Sleep for a given number of seconds.
    /// 
    /// This utility function pauses execution for the specified duration.
    /// Useful for creating delays in automation scripts or waiting
    /// for applications to respond.
    /// 
    /// # Arguments
    /// 
    /// * `seconds` - Duration to sleep in seconds (can be fractional)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mover_keyboard::Keyboard;
    /// 
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut keyboard = Keyboard::new()?;
    ///     
    ///     // Sleep for 1 second
    ///     keyboard.sleep(1.0);
    ///     
    ///     // Sleep for 500 milliseconds
    ///     keyboard.sleep(0.5);
    ///     
    ///     // Sleep for 2.5 seconds
    ///     keyboard.sleep(2.5);
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Common Use Cases
    /// 
    /// - **Waiting for applications**: Give time for apps to respond
    /// - **Human-like behavior**: Add realistic delays to automation
    /// - **Timing control**: Coordinate with other operations
    /// - **Debugging**: Slow down automation to observe behavior
    /// 
    /// # Performance Notes
    /// 
    /// - This method blocks the current thread
    /// - For non-blocking delays, consider using async alternatives
    /// - Very short delays (< 0.01 seconds) may not be precise
    pub fn sleep(&self, seconds: f64) {
        if seconds > 0.0 {
            thread::sleep(Duration::from_secs_f64(seconds));
        }
    }
}

/// Convert string key names to enigo Key codes.
/// 
/// This internal function maps human-readable key names to the `enigo` crate's
/// key codes. It supports a wide range of keys including letters, numbers,
/// special characters, function keys, and modifier keys.
/// 
/// # Supported Key Formats
/// 
/// - **Letters**: `"a"`, `"b"`, `"c"`, etc.
/// - **Numbers**: `"0"`, `"1"`, `"2"`, etc.
/// - **Special Keys**: `"enter"`, `"space"`, `"tab"`, etc.
/// - **Function Keys**: `"f1"`, `"f2"`, `"f3"`, etc.
/// - **Modifier Keys**: `"ctrl"`, `"shift"`, `"alt"`, etc.
/// - **Navigation Keys**: `"up"`, `"down"`, `"left"`, `"right"`
/// - **Symbols**: `"!"`, `"@"`, `"#"`, `"$"`, etc.
/// 
/// # Examples
/// 
/// ```rust
/// # use mover_keyboard::Keyboard;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let mut keyboard = Keyboard::new()?;
/// 
/// // All of these work:
/// keyboard.tap_key("a")?;           // Letter
/// keyboard.tap_key("1")?;           // Number
/// keyboard.tap_key("enter")?;       // Special key
/// keyboard.tap_key("f1")?;          // Function key
/// keyboard.tap_key("ctrl")?;        // Modifier key
/// keyboard.tap_key("up")?;          // Navigation key
/// keyboard.tap_key("!")?;           // Symbol
/// 
/// # Ok(())
/// # }
/// ```
/// 
/// # Error Handling
/// 
/// Returns an error if the key name is not recognized or supported.
/// The error message will indicate which key was not supported.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_creation() {
        let keyboard = Keyboard::new();
        assert!(keyboard.is_ok());
    }

    #[test]
    fn test_convert_key_basic() {
        // Test basic keys
        assert!(convert_key("a").is_ok());
        assert!(convert_key("1").is_ok());
        assert!(convert_key("!").is_ok());
    }

    #[test]
    fn test_convert_key_special() {
        // Test special keys
        assert!(convert_key("enter").is_ok());
        assert!(convert_key("space").is_ok());
        assert!(convert_key("tab").is_ok());
        assert!(convert_key("escape").is_ok());
    }

    #[test]
    fn test_convert_key_function() {
        // Test function keys
        assert!(convert_key("f1").is_ok());
        assert!(convert_key("f12").is_ok());
    }

    #[test]
    fn test_convert_key_modifier() {
        // Test modifier keys
        assert!(convert_key("ctrl").is_ok());
        assert!(convert_key("shift").is_ok());
        assert!(convert_key("alt").is_ok());
        assert!(convert_key("meta").is_ok());
    }

    #[test]
    fn test_convert_key_navigation() {
        // Test navigation keys
        assert!(convert_key("up").is_ok());
        assert!(convert_key("down").is_ok());
        assert!(convert_key("left").is_ok());
        assert!(convert_key("right").is_ok());
    }

    #[test]
    fn test_convert_key_case_insensitive() {
        // Test case insensitivity
        assert!(convert_key("ENTER").is_ok());
        assert!(convert_key("Enter").is_ok());
        assert!(convert_key("enter").is_ok());
    }

    #[test]
    fn test_convert_key_unsupported() {
        // Test unsupported keys
        assert!(convert_key("unsupported_key").is_err());
        assert!(convert_key("").is_err());
    }
}
