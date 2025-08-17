use mover::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the mover library
    mover::init()?;
    
    println!("{}", mover::info());
    
    // Get screen size
    let screen_size = mover_mouse::size()?;
    println!("Screen size: {}", screen_size);
    
    // Get current mouse position
    let current_pos = position()?;
    println!("Current mouse position: {}", current_pos);
    
    // Example: Move mouse to center of screen
    let center_x = screen_size.width / 2;
    let center_y = screen_size.height / 2;
    
    println!("Moving mouse to center of screen: ({}, {})", center_x, center_y);
    move_to(center_x, center_y)?;
    
    // Wait a moment
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    // Example: Click at center
    println!("Clicking at center");
    click(None)?;
    
    // Wait a moment
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    // Example: Move back to original position
    println!("Moving back to original position: {}", current_pos);
    move_to(current_pos.x, current_pos.y)?;
    
    println!("Demo completed successfully!");
    
    Ok(())
}
