# Mover Keyboard

A Rust library for cross-platform keyboard automation, inspired by PyAutoGUI but built with Rust's safety and performance. This crate provides comprehensive keyboard control functionality using the `enigo` crate for robust cross-platform support.

## 🚀 Features

- **Cross-Platform Support**: Works on Windows, macOS, and Linux
- **Real Keyboard Control**: Actual key presses, releases, and typing
- **Hotkey Support**: Complex key combinations and shortcuts
- **Comprehensive API**: All the keyboard functions you need for automation
- **Type Safety**: Rust's type system ensures safe keyboard operations
- **Error Handling**: Proper error handling with `Result` types
- **Instance-Based Design**: Clean, intuitive API using instances

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mover-keyboard = "0.1.0"
```

## 🎯 Quick Start

```rust
use mover_keyboard::Keyboard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a keyboard instance
    let mut keyboard = Keyboard::new()?;
    
    // Type some text
    keyboard.type_string("Hello, World!")?;
    
    // Press a key
    keyboard.press_key("enter")?;
    
    // Press a hotkey combination
    keyboard.press_hotkey(&["ctrl", "c"])?;
    
    Ok(())
}
```

## 📚 API Reference

### Creating a Keyboard Instance

```rust
use mover_keyboard::Keyboard;

// Create a new keyboard instance
let mut keyboard = Keyboard::new()?;
```

**Note**: The `mut` keyword is required because most keyboard operations modify internal state.

### Basic Typing

#### Type a String

```rust
// Type text instantly
keyboard.type_string("Hello, World!")?;

// Type text with delays between characters
keyboard.type_string_with_interval("Slow typing...", 0.5)?;
```

#### Individual Key Operations

```rust
// Press and release a key (tap)
keyboard.tap_key("enter")?;

// Press a key down (hold)
keyboard.press_key("shift")?;

// Release a key
keyboard.release_key("shift")?;

// Press a key multiple times
keyboard.press_key_multiple("a", 5, 0.2)?; // Press 'a' 5 times with 0.2s intervals
```

#### Press Multiple Keys

```rust
// Press keys in sequence
keyboard.press_keys(&["a", "b", "c"])?;
```

### Hotkey Combinations

```rust
// Common shortcuts
keyboard.press_hotkey(&["ctrl", "c"])?;    // Copy
keyboard.press_hotkey(&["ctrl", "v"])?;    // Paste
keyboard.press_hotkey(&["ctrl", "z"])?;    // Undo
keyboard.press_hotkey(&["ctrl", "a"])?;    // Select All

// Complex combinations
keyboard.press_hotkey(&["ctrl", "shift", "a"])?;  // Select All (alternative)
keyboard.press_hotkey(&["alt", "tab"])?;          // Switch windows
```

### Utility Functions

```rust
// Sleep for a given duration
keyboard.sleep(1.5); // Sleep for 1.5 seconds
```

## 🔑 Supported Keys

### Basic Keys
- **Letters**: `"a"`, `"b"`, `"c"`, etc.
- **Numbers**: `"0"`, `"1"`, `"2"`, etc.
- **Special Characters**: `"!"`, `"@"`, `"#"`, `"$"`, etc.

### Function Keys
- **F1-F12**: `"f1"`, `"f2"`, `"f3"`, etc.

### Navigation Keys
- **Arrow Keys**: `"up"`, `"down"`, `"left"`, `"right"`
- **Page Navigation**: `"home"`, `"end"`, `"pageup"`, `"pagedown"`

### Modifier Keys
- **Control**: `"ctrl"` or `"control"`
- **Shift**: `"shift"`
- **Alt**: `"alt"`
- **Meta/Windows/Command**: `"meta"`, `"win"`, or `"command"`

### Special Keys
- **Enter**: `"enter"` or `"return"`
- **Space**: `"space"`
- **Tab**: `"tab"`
- **Escape**: `"escape"` or `"esc"`
- **Backspace**: `"backspace"`
- **Delete**: `"delete"` or `"del"`

## 🎨 Advanced Examples

### Automated Form Filling

```rust
use mover_keyboard::Keyboard;

fn fill_form() -> Result<(), Box<dyn std::error::Error>> {
    let mut keyboard = Keyboard::new()?;
    
    // Fill in name field
    keyboard.type_string("John Doe")?;
    keyboard.press_key("tab")?;
    
    // Fill in email field
    keyboard.type_string("john@example.com")?;
    keyboard.press_key("tab")?;
    
    // Fill in phone field
    keyboard.type_string("555-1234")?;
    keyboard.press_key("enter")?;
    
    Ok(())
}
```

### Text Editing Automation

```rust
use mover_keyboard::Keyboard;

fn edit_text() -> Result<(), Box<dyn std::error::Error>> {
    let mut keyboard = Keyboard::new()?;
    
    // Select all text
    keyboard.press_hotkey(&["ctrl", "a"])?;
    keyboard.sleep(0.1);
    
    // Copy selected text
    keyboard.press_hotkey(&["ctrl", "c"])?;
    keyboard.sleep(0.1);
    
    // Move to end and paste
    keyboard.press_key("end")?;
    keyboard.press_key("enter")?;
    keyboard.press_hotkey(&["ctrl", "v"])?;
    
    Ok(())
}
```

### Gaming Automation

```rust
use mover_keyboard::Keyboard;

fn game_automation() -> Result<(), Box<dyn std::error::Error>> {
    let mut keyboard = Keyboard::new()?;
    
    // Press movement keys
    keyboard.press_key("w")?;  // Move forward
    keyboard.sleep(0.5);
    keyboard.release_key("w")?;
    
    // Jump
    keyboard.press_key("space")?;
    keyboard.sleep(0.1);
    keyboard.release_key("space")?;
    
    // Use ability
    keyboard.press_key("q")?;
    keyboard.sleep(0.1);
    keyboard.release_key("q")?;
    
    Ok(())
}
```

## 🧪 Running Examples

The crate includes a comprehensive demo that showcases all features:

```bash
# Run the keyboard demo
cargo run --example keyboard_demo

# Check if the example compiles
cargo check --example keyboard_demo
```

## ⚠️ Important Notes

### Safety Warnings
- **This library controls your actual keyboard** - use with caution
- **Always test automation scripts in safe environments**
- **Be aware of your system's security settings**
- **Some applications may block keyboard automation**

### Performance Considerations
- **Text typing** is optimized for speed using `enigo`'s native text method
- **Individual key presses** are slower but more precise
- **Hotkey combinations** have small delays to ensure reliability

### Platform Differences
- **Windows**: Full support for all features
- **macOS**: Full support, may require accessibility permissions
- **Linux**: Full support, may require X11 or Wayland setup

## 🔧 Error Handling

All methods return `Result<T, Box<dyn std::error::Error>>` for robust error handling:

```rust
match keyboard.type_string("Hello") {
    Ok(()) => println!("Text typed successfully"),
    Err(e) => eprintln!("Failed to type text: {}", e),
}
```

## 🚀 Roadmap

- [ ] **Recording and Playback**: Record keyboard sequences and replay them
- [ ] **Macro Support**: Create and execute complex keyboard macros
- [ ] **Timing Control**: More precise control over delays and timing
- [ ] **Event Hooks**: Listen for keyboard events
- [ ] **Configuration**: Customizable key mappings and settings

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/mover.git
cd mover

# Check if keyboard demo compiles
cargo check -p mover-keyboard --example keyboard_demo

# Run tests
cargo test -p mover-keyboard
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **PyAutoGUI**: For inspiration and API design
- **Enigo**: For robust cross-platform keyboard control
- **Rust Community**: For excellent tooling and ecosystem

## 📞 Support

If you encounter any issues or have questions:

1. **Check the examples** in the `examples/` directory
2. **Search existing issues** on GitHub
3. **Create a new issue** with detailed information about your problem
4. **Join our community** discussions

---

**Happy automating!** 🎉
