//! Utility functions and helpers for the mover automation library
//! 
//! This module provides various utility functions, configuration management,
//! and helper functions for the mover library.

use mover_core::{MoverResult, Point, Size, MouseButton};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use std::io::Write;

/// Configuration for the mover library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoverConfig {
    /// Global pause between actions (in seconds)
    pub pause: f64,
    
    /// Failsafe enabled
    pub failsafe: bool,
    
    /// Failsafe points (screen corners)
    pub failsafe_points: Vec<Point>,
    
    /// Minimum duration for mouse movements
    pub minimum_duration: f64,
    
    /// Minimum sleep time
    pub minimum_sleep: f64,
    
    /// Log screenshots
    pub log_screenshots: bool,
    
    /// Screenshot log limit
    pub screenshot_log_limit: Option<usize>,
}

impl Default for MoverConfig {
    fn default() -> Self {
        Self {
            pause: 0.1,
            failsafe: true,
            failsafe_points: vec![
                Point::new(0, 0),
                Point::new(0, 0), // Will be set to actual screen corners
            ],
            minimum_duration: 0.1,
            minimum_sleep: 0.05,
            log_screenshots: false,
            screenshot_log_limit: Some(10),
        }
    }
}

/// Failsafe manager
pub struct FailsafeManager {
    config: MoverConfig,
    last_check: Instant,
}

impl FailsafeManager {
    /// Creates a new failsafe manager
    pub fn new(config: MoverConfig) -> Self {
        Self {
            config,
            last_check: Instant::now(),
        }
    }
    
    /// Checks if failsafe should be triggered
    pub fn check(&mut self) -> MoverResult<()> {
        if !self.config.failsafe {
            return Ok(());
        }
        
        let current_pos = mover_core::platform::get_platform()?.get_position()?;
        
        if self.config.failsafe_points.contains(&current_pos) {
            return Err(mover_core::MoverError::FailsafeTriggered(
                "Mouse moved to failsafe position".to_string()
            ));
        }
        
        self.last_check = Instant::now();
        Ok(())
    }
    
    /// Updates failsafe points based on screen size
    pub fn update_failsafe_points(&mut self, screen_size: Size) {
        self.config.failsafe_points = vec![
            Point::new(0, 0),                                    // Top-left
            Point::new(screen_size.width - 1, 0),                 // Top-right
            Point::new(0, screen_size.height - 1),                // Bottom-left
            Point::new(screen_size.width - 1, screen_size.height - 1), // Bottom-right
        ];
    }
}

/// Action recorder for automation scripts
pub struct ActionRecorder {
    actions: Vec<RecordedAction>,
    start_time: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordedAction {
    MouseMove { x: i32, y: i32, duration: f64 },
    MouseClick { x: i32, y: i32, button: MouseButton },
    MouseDrag { from: Point, to: Point, button: MouseButton },
    KeyPress { key: String },
    KeyType { text: String },
    Screenshot { path: String },
    Wait { duration: f64 },
}

impl ActionRecorder {
    /// Creates a new action recorder
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            start_time: Instant::now(),
        }
    }
    
    /// Records a mouse move action
    pub fn record_mouse_move(&mut self, x: i32, y: i32, duration: f64) {
        self.actions.push(RecordedAction::MouseMove { x, y, duration });
    }
    
    /// Records a mouse click action
    pub fn record_mouse_click(&mut self, x: i32, y: i32, button: MouseButton) {
        self.actions.push(RecordedAction::MouseClick { x, y, button });
    }
    
    /// Records a mouse drag action
    pub fn record_mouse_drag(&mut self, from: Point, to: Point, button: MouseButton) {
        self.actions.push(RecordedAction::MouseDrag { from, to, button });
    }
    
    /// Records a key press action
    pub fn record_key_press(&mut self, key: String) {
        self.actions.push(RecordedAction::KeyPress { key });
    }
    
    /// Records a key type action
    pub fn record_key_type(&mut self, text: String) {
        self.actions.push(RecordedAction::KeyType { text });
    }
    
    /// Records a screenshot action
    pub fn record_screenshot(&mut self, path: String) {
        self.actions.push(RecordedAction::Screenshot { path });
    }
    
    /// Records a wait action
    pub fn record_wait(&mut self, duration: f64) {
        self.actions.push(RecordedAction::Wait { duration });
    }
    
    /// Gets all recorded actions
    pub fn get_actions(&self) -> &[RecordedAction] {
        &self.actions
    }
    
    /// Exports actions to JSON
    pub fn export_json(&self) -> MoverResult<String> {
        serde_json::to_string_pretty(&self.actions)
            .map_err(|e| mover_core::MoverError::Other(format!("Failed to serialize actions: {}", e)))
    }
    
    /// Gets the total recording time
    pub fn get_total_time(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// Utility functions for common operations
pub mod functions {
    use super::*;
    
    /// Sleep for a given number of seconds
    pub fn sleep(seconds: f64) {
        if seconds > 0.0 {
            std::thread::sleep(Duration::from_secs_f64(seconds));
        }
    }
    
    /// Countdown timer
    pub fn countdown(seconds: u32) {
        for i in (1..=seconds).rev() {
            print!("{} ", i);
            std::io::stdout().flush().ok();
            std::thread::sleep(Duration::from_secs(1));
        }
        println!("0");
    }
    
    /// Gets a point on a line between two points at a given proportion
    pub fn get_point_on_line(x1: i32, y1: i32, x2: i32, y2: i32, n: f64) -> Point {
        let x = ((x2 - x1) as f64 * n) + x1 as f64;
        let y = ((y2 - y1) as f64 * n) + y1 as f64;
        Point::new(x as i32, y as i32)
    }
    
    /// Calculates distance between two points
    pub fn distance_between_points(p1: &Point, p2: &Point) -> f64 {
        let dx = (p1.x - p2.x) as f64;
        let dy = (p1.y - p2.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
    
    /// Checks if a point is within a rectangle
    pub fn point_in_rect(point: &Point, x: i32, y: i32, width: u32, height: u32) -> bool {
        point.x >= x && point.x < x + width as i32 && 
        point.y >= y && point.y < y + height as i32
    }
}

/// Re-export commonly used items
pub use functions::*;
