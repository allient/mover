use mover_mouse::*;
use mover_core::types::MouseButton;
use std::time::Duration;
use std::f64::consts::PI;
use std::thread;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🖱️  MOVER MOUSE DEMO");
    println!("=====================");
    println!("This demo will showcase various mouse control features.");
    println!("⚠️  WARNING: Your mouse will move automatically!");
    println!("   Make sure you have a safe area to test.");
    println!("   Press Ctrl+C to stop at any time.\n");

    // Give user time to prepare
    countdown(5);

    // Run all demo sections
    demo_basic_info()?;
    demo_movements()?;
    demo_clicking()?;
    demo_scrolling()?;
    demo_advanced_patterns()?;
    demo_circular_movement()?;
    demo_sine_wave_movement()?;

    println!("\n🎉 All demos completed successfully!");
    println!("Thank you for trying out mover-mouse!");

    Ok(())
}

fn demo_basic_info() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 SECTION 1: Basic Information");
    println!("----------------------------------------");

    // Get current mouse position
    let pos = position()?;
    println!("📍 Current mouse position: ({}, {})", pos.x, pos.y);

    // Get screen size
    let screen = size()?;
    println!("🖥️  Screen resolution: {}x{}", screen.width, screen.height);

    // Calculate center
    let center_x = screen.width / 2;
    let center_y = screen.height / 2;
    println!("🎯 Screen center: ({}, {})", center_x, center_y);

    // Check if current position is on screen
    let on_screen = on_screen(pos.x, pos.y)?;
    println!("✅ Current position on screen: {}", on_screen);

    Ok(())
}

fn demo_movements() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚀 SECTION 2: Mouse Movements");
    println!("----------------------------------------");

    let screen = size()?;
    let center_x = screen.width / 2;
    let center_y = screen.height / 2;

    println!("🔄 Moving to screen center...");
    move_to(center_x, center_y)?;
    thread::sleep(Duration::from_millis(500));

    println!("⬆️  Moving up 100 pixels...");
    move_by(0, -100)?;
    thread::sleep(Duration::from_millis(500));

    println!("➡️  Moving right 100 pixels...");
    move_by(100, 0)?;
    thread::sleep(Duration::from_millis(500));

    println!("⬇️  Moving down 100 pixels...");
    move_by(0, 100)?;
    thread::sleep(Duration::from_millis(500));

    println!("⬅️  Moving left 100 pixels...");
    move_by(-100, 0)?;
    thread::sleep(Duration::from_millis(500));

    println!("🎯 Returning to center...");
    move_to(center_x, center_y)?;
    thread::sleep(Duration::from_millis(500));

    Ok(())
}

fn demo_clicking() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🖱️  SECTION 3: Mouse Clicking");
    println!("----------------------------------------");

    let screen = size()?;
    let center_x = screen.width / 2;
    let center_y = screen.height / 2;

    println!("👆 Left click at center...");
    move_to(center_x, center_y)?;
    left_click()?;
    thread::sleep(Duration::from_millis(500));

    println!("👆 Right click at center...");
    move_to(center_x, center_y)?;
    right_click()?;
    thread::sleep(Duration::from_millis(500));

    println!("👆 Middle click at center...");
    move_to(center_x, center_y)?;
    middle_click()?;
    thread::sleep(Duration::from_millis(500));

    println!("👆👆 Double click at center...");
    move_to(center_x, center_y)?;
    double_click(Some(MouseButton::Left))?;
    thread::sleep(Duration::from_millis(500));

    println!("👆👆👆 Triple click at center...");
    move_to(center_x, center_y)?;
    triple_click(Some(MouseButton::Left))?;
    thread::sleep(Duration::from_millis(500));

    Ok(())
}

fn demo_scrolling() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📜 SECTION 4: Mouse Scrolling");
    println!("----------------------------------------");

    let screen = size()?;
    let center_x = screen.width / 2;
    let center_y = screen.height / 2;

    println!("🔄 Moving to center for scrolling...");
    move_to(center_x, center_y)?;
    thread::sleep(Duration::from_millis(500));

    println!("⬆️  Scrolling up 3 clicks...");
    scroll(3)?;
    thread::sleep(Duration::from_millis(500));

    println!("⬇️  Scrolling down 3 clicks...");
    scroll(-3)?;
    thread::sleep(Duration::from_millis(500));

    println!("➡️  Scrolling right 2 clicks...");
    hscroll(2)?;
    thread::sleep(Duration::from_millis(500));

    println!("⬅️  Scrolling left 2 clicks...");
    hscroll(-2)?;
    thread::sleep(Duration::from_millis(500));

    Ok(())
}

fn demo_advanced_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 SECTION 5: Advanced Movement Patterns");
    println!("----------------------------------------");

    let screen = size()?;
    let center_x = screen.width / 2;
    let center_y = screen.height / 2;

    println!("🔄 Moving to center...");
    move_to(center_x, center_y)?;
    thread::sleep(Duration::from_millis(500));

    // Draw a square
    println!("⬜ Drawing a square...");
    let size = 100;
    let half_size = size / 2;
    
    // Top-left corner
    move_to(center_x - half_size, center_y - half_size)?;
    thread::sleep(Duration::from_millis(100));
    
    // Top-right corner
    move_to(center_x + half_size, center_y - half_size)?;
    thread::sleep(Duration::from_millis(100));
    
    // Bottom-right corner
    move_to(center_x + half_size, center_y + half_size)?;
    thread::sleep(Duration::from_millis(100));
    
    // Bottom-left corner
    move_to(center_x - half_size, center_y + half_size)?;
    thread::sleep(Duration::from_millis(100));
    
    // Back to center
    move_to(center_x, center_y)?;
    thread::sleep(Duration::from_millis(500));

    Ok(())
}

fn demo_circular_movement() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⭕ SECTION 6: Circular Movement");
    println!("----------------------------------------");

    let screen = size()?;
    let center_x = screen.width / 2;
    let center_y = screen.height / 2;
    let radius = 100;

    println!("🔄 Moving to center...");
    move_to(center_x, center_y)?;
    thread::sleep(Duration::from_millis(500));

    println!("⭕ Drawing clockwise circle (3 seconds)...");
    draw_circle(center_x, center_y, radius, true, 3.0)?;
    thread::sleep(Duration::from_millis(500));

    println!("⭕ Drawing counter-clockwise circle (3 seconds)...");
    draw_circle(center_x, center_y, radius, false, 3.0)?;
    thread::sleep(Duration::from_millis(500));

    println!("🎯 Returning to center...");
    move_to(center_x, center_y)?;
    thread::sleep(Duration::from_millis(500));

    Ok(())
}

fn demo_sine_wave_movement() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌊 SECTION 7: Sine Wave Movement");
    println!("----------------------------------------");

    let screen = size()?;
    let center_x = screen.width / 2;
    let center_y = screen.height / 2;
    let amplitude = 80; // pixels for vertical oscillation
    let wave_width = 400; // pixels for horizontal wave length
    let duration = 8.0; // seconds

    // Calculate starting position (left side of wave)
    let start_x = center_x - wave_width / 2;
    let start_y = center_y;

    // Move to starting position
    move_to(start_x, start_y)?;
    thread::sleep(Duration::from_millis(500));

    println!("🌊 Drawing sine wave pattern (8 seconds)...");
    println!("   📏 Wave width: {} pixels", wave_width);
    println!("   📐 Amplitude: {} pixels", amplitude);

    // Draw the sine wave by moving through points
    let num_points = 200; // Number of points for smooth wave
    let time_per_point = duration / num_points as f64;

    for i in 0..=num_points {
        let progress = i as f64 / num_points as f64;
        
        // X position moves linearly from left to right
        let x = start_x + (progress * wave_width as f64) as i32;
        
        // Y position follows sine function for smooth oscillation
        let angle = progress * 2.0 * PI * 2.0; // 2 complete cycles
        let y = center_y + (amplitude as f64 * angle.sin()) as i32;
        
        if i == 0 {
            // First point - just move there
            move_to(x, y)?;
        } else {
            // Subsequent points - drag to create continuous wave line
            move_to(x, y)?;
        }
        
        // Smooth timing for the wave
        thread::sleep(Duration::from_millis((time_per_point * 1000.0) as u64));
    }

    println!("✅ Sine wave movement completed!");

    // Return to center
    move_to(center_x, center_y)?;
    thread::sleep(Duration::from_millis(500));

    Ok(())
}

/// Draws a circle by moving the mouse in a circular pattern
fn draw_circle(center_x: i32, center_y: i32, radius: i32, clockwise: bool, duration: f64) -> Result<(), Box<dyn std::error::Error>> {
    let num_points = 36; // 36 points = 10° intervals
    let total_angle = 2.0 * PI;
    
    // Calculate time per point
    let time_per_point = duration / num_points as f64;
    
    for i in 0..=num_points {
        let angle = if clockwise {
            i as f64 * total_angle / num_points as f64
        } else {
            (num_points - i) as f64 * total_angle / num_points as f64
        };
        
        let x = center_x + (radius as f64 * angle.cos()) as i32;
        let y = center_y + (radius as f64 * angle.sin()) as i32;
        
        if i == 0 {
            // First point - just move there
            move_to(x, y)?;
        } else {
            // Subsequent points - drag to create continuous line
            drag_to(x, y, Some(MouseButton::Left))?;
        }
        
        // Small delay for smooth animation
        thread::sleep(Duration::from_millis((time_per_point * 1000.0) as u64));
    }
    
    Ok(())
}

/// Helper function for countdown
fn countdown(seconds: u32) {
    for i in (1..=seconds).rev() {
        print!("{} ", i);
        std::io::stdout().flush().ok();
        thread::sleep(Duration::from_secs(1));
    }
    println!("0");
}
