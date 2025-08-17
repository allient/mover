//! Common types used throughout the mover library

use std::fmt;

/// A 2D point with x and y coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Create a new point with the given coordinates
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Get the distance to another point
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    /// Check if this point is on screen
    pub fn is_on_screen(&self, screen_size: &Size) -> bool {
        self.x >= 0 && self.x < screen_size.width && self.y >= 0 && self.y < screen_size.height
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (i32, i32) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Screen dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    /// Create a new size with the given dimensions
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    /// Get the area of the screen
    pub fn area(&self) -> i64 {
        self.width as i64 * self.height as i64
    }

    /// Check if a point is within this size
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

impl From<(i32, i32)> for Size {
    fn from((width, height): (i32, i32)) -> Self {
        Self { width, height }
    }
}

impl From<Size> for (i32, i32) {
    fn from(size: Size) -> Self {
        (size.width, size.height)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

/// Mouse button types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Primary,
    Secondary,
    Button4,
    Button5,
    Button6,
    Button7,
}

impl MouseButton {
    /// Get the button number (for X11 compatibility)
    pub fn number(&self) -> u8 {
        match self {
            MouseButton::Left => 1,
            MouseButton::Middle => 2,
            MouseButton::Right => 3,
            MouseButton::Primary => 1, // Will be resolved based on OS settings
            MouseButton::Secondary => 3, // Will be resolved based on OS settings
            MouseButton::Button4 => 4,
            MouseButton::Button5 => 5,
            MouseButton::Button6 => 6,
            MouseButton::Button7 => 7,
        }
    }

    /// Check if this is a valid button for the current platform
    pub fn is_valid_for_platform(&self) -> bool {
        #[cfg(target_os = "linux")]
        {
            matches!(self, 
                MouseButton::Left | MouseButton::Middle | MouseButton::Right |
                MouseButton::Primary | MouseButton::Secondary |
                MouseButton::Button4 | MouseButton::Button5 | MouseButton::Button6 | MouseButton::Button7
            )
        }
        
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            matches!(self, 
                MouseButton::Left | MouseButton::Middle | MouseButton::Right |
                MouseButton::Primary | MouseButton::Secondary
            )
        }
    }
}

impl Default for MouseButton {
    fn default() -> Self {
        MouseButton::Primary
    }
}

impl fmt::Display for MouseButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MouseButton::Left => write!(f, "left"),
            MouseButton::Middle => write!(f, "middle"),
            MouseButton::Right => write!(f, "right"),
            MouseButton::Primary => write!(f, "primary"),
            MouseButton::Secondary => write!(f, "secondary"),
            MouseButton::Button4 => write!(f, "button4"),
            MouseButton::Button5 => write!(f, "button5"),
            MouseButton::Button6 => write!(f, "button6"),
            MouseButton::Button7 => write!(f, "button7"),
        }
    }
}

/// Scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

impl ScrollDirection {
    /// Get the scroll value (positive for up/right, negative for down/left)
    pub fn value(&self) -> i32 {
        match self {
            ScrollDirection::Up | ScrollDirection::Right => 1,
            ScrollDirection::Down | ScrollDirection::Left => -1,
        }
    }
}

impl fmt::Display for ScrollDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScrollDirection::Up => write!(f, "up"),
            ScrollDirection::Down => write!(f, "down"),
            ScrollDirection::Left => write!(f, "left"),
            ScrollDirection::Right => write!(f, "right"),
        }
    }
}

/// Tweening function for smooth mouse movements
pub type TweenFn = fn(f64) -> f64;

/// Linear tweening function (default)
pub fn linear_tween(t: f64) -> f64 {
    t
}

/// Ease-in quadratic tweening
pub fn ease_in_quad(t: f64) -> f64 {
    t * t
}

/// Ease-out quadratic tweening
pub fn ease_out_quad(t: f64) -> f64 {
    t * (2.0 - t)
}

/// Ease-in-out quadratic tweening
pub fn ease_in_out_quad(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}
