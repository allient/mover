# Mover

A Rust implementation of PyAutoGUI, providing cross-platform automation capabilities for controlling the mouse and keyboard programmatically.

## Overview

Mover is a Rust library that allows you to control the mouse and keyboard to automate interactions with other applications. It's inspired by and aims to provide the same functionality as [PyAutoGUI](https://deepwiki.com/asweigart/pyautogui/1-overview), but written in Rust for better performance and safety.

## Project Structure

This is a monorepo containing multiple crates:

- **`mover-core`** - Core types, traits, and platform abstractions
- **`mover-mouse`** - Mouse control functionality (movement, clicking, dragging, scrolling)
- **`mover-keyboard`** - Keyboard control functionality (typing, key presses, hotkeys)
- **`mover-screen`** - Screen capture and image recognition
- **`mover-utils`** - Utility functions, configuration, and helpers
- **`mover-macros`** - Procedural macros for automation scripts
- **`mover`** - Main crate that brings everything together

## Features

- **Cross-platform support**: Works on Windows, macOS, and Linux
- **Mouse control**: Move, click, drag, scroll, and get position
- **Keyboard control**: Type text, press keys, hotkeys, and shortcuts
- **Screen capture**: Take screenshots and find images on screen
- **Mathematical movement patterns**: Perfect circles, sine waves, spirals, and custom geometric patterns
- **Smooth animations**: Tweening functions for natural mouse movements
- **Safety features**: Built-in failsafe to prevent runaway automation
- **High performance**: Rust's zero-cost abstractions and memory safety
- **Modular design**: Use only the modules you need
- **Procedural macros**: Convenient macros for automation scripts
- **Precision control**: Mathematical precision for artistic and scientific applications

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mover = "0.1.0"
```

Or use specific modules:

```toml
[dependencies]
mover-core = "0.1.0"
mover-mouse = "0.1.0"
mover-keyboard = "0.1.0"
mover-screen = "0.1.0"
```

## Quick Start

```rust
use mover::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the library
    mover::init()?;
    
    // Move mouse to coordinates
    move_to(100, 200)?;
    
    // Click at current position
    click(None)?;
    
    // Type some text
    type_string("Hello, World!")?;
    
    // Take a screenshot
    let screenshot = capture()?;
    
    Ok(())
}
```

## Examples

### Basic Mouse Control

```rust
use mover_mouse::*;

// Move mouse to specific coordinates
move_to(500, 300)?;

// Click at current position
click(None)?;

// Right click
right_click()?;

// Double click
double_click(None)?;

// Drag from one point to another
drag_to(200, 200, None)?;

// Scroll
scroll(3)?;
```

### Keyboard Automation

```rust
use mover_keyboard::*;

// Type a string
type_string("Hello, World!")?;

// Press a single key
press_key("enter")?;

// Press multiple keys
press_keys(&["ctrl", "c"])?;

// Press hotkey combination
press_hotkey(&["ctrl", "shift", "c"])?;
```

### Screen Capture

```rust
use mover_screen::*;

// Take a screenshot
let screenshot = capture()?;

// Save screenshot to file
save_screenshot("screenshot.png")?;

// Get pixel color
let color = get_pixel_color(100, 100)?;
println!("Pixel color: RGB({}, {}, {})", color.0, color.1, color.2);
```

### Using Macros

```rust
use mover_macros::*;

// Mouse movement sequence
let _: Result<(), _> = mouse_sequence!((100, 100), (200, 200), (300, 300));

// Click sequence
let _: Result<(), _> = click_sequence!((100, 100, Left), (200, 200, Right));

// Keyboard sequence
let _: Result<(), _> = keyboard_sequence!("ctrl", "c");

// Wait sequence
wait_sequence!(1.0, 0.5, 2.0);
```

### Advanced Mathematical Movement Patterns

Mover supports sophisticated mathematical movement patterns for precise automation:

#### Perfect Circles
```rust
use mover_mouse::*;
use std::f64::consts::PI;

fn draw_circle(center_x: i32, center_y: i32, radius: i32, duration: f64) -> Result<(), Box<dyn std::error::Error>> {
    let num_points = 36; // 36 points = 10° intervals
    let time_per_point = duration / num_points as f64;
    
    // Start at right side of circle
    let start_x = center_x + radius;
    move_to(start_x, center_y)?;
    
    for i in 0..=num_points {
        let angle = i as f64 * 2.0 * PI / num_points as f64;
        let x = center_x + (radius as f64 * angle.cos()) as i32;
        let y = center_y + (radius as f64 * angle.sin()) as i32;
        
        if i == 0 {
            move_to(x, y)?;
        } else {
            drag_to(x, y, Some(MouseButton::Left))?;
        }
        
        sleep(Duration::from_millis((time_per_point * 1000.0) as u64));
    }
    
    Ok(())
}
```

#### Sine Wave Patterns
```rust
fn draw_sine_wave(center_x: i32, center_y: i32, amplitude: i32, wave_width: i32, cycles: f64, duration: f64) -> Result<(), Box<dyn std::error::Error>> {
    let num_points = 200; // Smooth wave with 200 points
    let time_per_point = duration / num_points as f64;
    
    let start_x = center_x - wave_width / 2;
    move_to(start_x, center_y)?;
    
    for i in 0..=num_points {
        let progress = i as f64 / num_points as f64;
        let x = start_x + (progress * wave_width as f64) as i32;
        let angle = progress * 2.0 * PI * cycles;
        let y = center_y + (amplitude as f64 * angle.sin()) as i32;
        
        move_to(x, y)?;
        sleep(Duration::from_millis((time_per_point * 1000.0) as u64));
    }
    
    Ok(())
}
```

#### Custom Geometric Patterns
```rust
// Spiral pattern
fn draw_spiral(center_x: i32, center_y: i32, max_radius: i32, duration: f64) -> Result<(), Box<dyn std::error::Error>> {
    let num_points = 100;
    let time_per_point = duration / num_points as f64;
    
    for i in 0..=num_points {
        let progress = i as f64 / num_points as f64;
        let radius = progress * max_radius as f64;
        let angle = progress * 4.0 * PI; // 2 full rotations
        
        let x = center_x + (radius * angle.cos()) as i32;
        let y = center_y + (radius * angle.sin()) as i32;
        
        if i == 0 {
            move_to(x, y)?;
        } else {
            move_to(x, y)?;
        }
        
        sleep(Duration::from_millis((time_per_point * 1000.0) as u64));
    }
    
    Ok(())
}
```

These patterns are perfect for:
- **🎨 Artistic drawing applications**
- **📊 Data visualization demonstrations**
- **🎮 Game automation and testing**
- **🔬 Scientific visualization tools**
- **🎭 Performance art and installations**

## Running Examples

### Enhanced Mouse Demo

The `mover-mouse` crate includes a spectacular demo showcasing all capabilities:

```bash
cargo run -p mover-mouse --example mouse_demo
```

This enhanced demo demonstrates:

#### **📊 Section 1: Basic Information**
- Current mouse position and screen dimensions
- On-screen validation and center calculation

#### **🚀 Section 2: Movement Demonstrations**
- Smooth animations with tweening functions
- Relative movement patterns (square pattern)
- Smooth transitions between movements

#### **🖱️ Section 3: Clicking Demonstrations**
- All click types: left, right, middle
- Double and triple clicks with proper timing

#### **📜 Section 4: Scrolling Demonstrations**
- Vertical and horizontal scrolling
- Different scroll intensities

#### **🎨 Section 5: Advanced Patterns**
- Triangle drawing using drag operations
- Line interpolation with utility functions
- Percentage-based point calculations

#### **⭕ Section 6: Circular Movement**
- **First Circle**: Clockwise movement (5 seconds)
- **Second Circle**: Counter-clockwise movement (5 seconds)
- Perfect mathematical circles using trigonometry
- Continuous line drawing with drag operations

#### **🌊 Section 7: Sine Wave Movement**
- Mathematical sine wave pattern (8 seconds)
- 400-pixel wide wave with 80-pixel amplitude
- 2 complete sine wave cycles
- Smooth horizontal movement with vertical oscillation

#### **🔄 Return to Start**
- Always returns to the original mouse position

The demo provides a spectacular visual demonstration of the crate's capabilities, including the mathematical precision of circular and sine wave movements!

### Other Examples

```bash
# Basic mouse operations
cargo run -p mover-mouse --example basic_mouse

# Keyboard automation
cargo run -p mover-keyboard --example keyboard_demo

# Screen capture
cargo run -p mover-screen --example screen_demo
```

## Safety Features

Mover includes several safety features to prevent automation from running out of control:

- **Failsafe**: Move your mouse to the upper-left corner of the screen to stop automation
- **Pause between actions**: Configurable delays to prevent actions from happening too quickly
- **Error handling**: Graceful fallbacks when operations fail
- **Platform validation**: Checks for supported operations on each platform

## Platform Support

- **Windows**: Uses Windows API for mouse and keyboard control
- **macOS**: Uses Core Graphics and Accessibility frameworks
- **Linux**: Uses X11 or Wayland depending on the display server

## Configuration

```rust
use mover_utils::MoverConfig;

let config = MoverConfig {
    pause: 0.1,
    failsafe: true,
    minimum_duration: 0.1,
    ..Default::default()
};
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Clone the repository
2. Install Rust toolchain
3. Run tests: `cargo test --workspace`
4. Build all crates: `cargo build --workspace`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [PyAutoGUI](https://github.com/asweigart/pyautogui) by Al Sweigart
- Built with Rust for performance and safety
- Cross-platform automation made simple

## Roadmap

- [x] Monorepo structure
- [x] Core types and traits
- [x] Mouse control implementation
- [x] Keyboard control implementation
- [x] Screen capture functionality
- [x] Utility functions and configuration
- [x] Procedural macros
- [x] Mathematical movement patterns (circles, sine waves, spirals)
- [x] Enhanced demo with 7 comprehensive sections
- [ ] Platform-specific implementations (Windows, macOS, Linux)
- [ ] Image recognition capabilities
- [ ] Cross-platform testing
- [ ] Performance optimizations
- [ ] Additional automation features

---

**Note**: This project is currently in active development with significant progress made. The API is stable for core functionality, and we've implemented advanced mathematical movement patterns including perfect circles, sine waves, and custom geometric patterns. The enhanced demo showcases all capabilities across 7 comprehensive sections. Contributions and feedback are welcome as we continue to expand the platform-specific implementations and add more automation features.
