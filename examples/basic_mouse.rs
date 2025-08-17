//! Basic mouse control example
//! 
//! This example demonstrates basic mouse operations using the mover library.

use mover::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the mover library
    mover::init()?;
    
    println!("Basic Mouse Control Example");
    println!("==========================");
    
    // Get screen size
    let screen_size = size()?;
    println!("Screen size: {}", screen_size);
    
    // Get current mouse position
    let start_pos = position()?;
    println!("Starting position: {}", start_pos);
    
    // Example 1: Move to different positions
    println!("\n1. Moving to different positions...");
    
    let positions = [
        (100, 100),
        (200, 150),
        (300, 200),
        (400, 250),
    ];
    
    for (i, (x, y)) in positions.iter().enumerate() {
        println!("   Moving to position {}: ({}, {})", i + 1, x, y);
        move_to(*x, *y)?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    
    // Example 2: Relative movements
    println!("\n2. Relative movements...");
    
    println!("   Moving 50 pixels right");
    move_by(50, 0)?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    println!("   Moving 50 pixels down");
    move_by(0, 50)?;
    std::thread::time::sleep(std::time::Duration::from_millis(500));
    
    println!("   Moving 50 pixels left");
    move_by(-50, 0)?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    println!("   Moving 50 pixels up");
    move_by(0, -50)?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Example 3: Clicking
    println!("\n3. Clicking operations...");
    
    let current_pos = position()?;
    
    println!("   Left click at current position");
    left_click()?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    println!("   Right click at current position");
    right_click()?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    println!("   Double click at current position");
    double_click(None)?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Example 4: Scrolling
    println!("\n4. Scrolling...");
    
    println!("   Scrolling up");
    scroll(3)?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    println!("   Scrolling down");
    scroll(-3)?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // Example 5: Return to start
    println!("\n5. Returning to start position...");
    move_to(start_pos.x, start_pos.y)?;
    
    println!("Example completed successfully!");
    println!("Final position: {}", position()?);
    
    Ok(())
}
