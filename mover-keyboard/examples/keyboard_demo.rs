//! Example demonstrating the keyboard control functionality using enigo
//! 
//! This example shows how to use the refactored mover-keyboard crate
//! which now uses enigo for actual keyboard control instead of placeholder functions.

use mover_keyboard::Keyboard;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⌨️  Mover Keyboard Demo - Using Enigo for cross-platform keyboard control");
    println!("===============================================================");
    
    println!("⚠️  WARNING: This demo will control your keyboard!");
    println!("   Make sure you have a text editor or notepad open to see the results.");
    println!("   You can stop the demo by pressing Ctrl+C in this terminal.");
    println!();
    
    // Create keyboard instance
    let mut keyboard = Keyboard::new()?;
    
    // Give user time to prepare
    println!("🔄 Starting in 5 seconds...");
    countdown(5);
    
    // Section 1: Basic Typing
    demo_basic_typing(&mut keyboard)?;
    
    // Section 2: Key Presses
    demo_key_presses(&mut keyboard)?;
    
    // Section 3: Hotkey Combinations
    demo_hotkeys(&mut keyboard)?;
    
    // Section 4: Advanced Patterns
    demo_advanced_patterns(&mut keyboard)?;
    
    println!("\n✅ Keyboard demo completed successfully!");
    println!("📝 Check your text editor to see the results!");
    
    Ok(())
}

fn demo_basic_typing(keyboard: &mut Keyboard) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📝 SECTION 1: Basic Typing");
    println!("----------------------------------------");
    
    // Type a simple message
    println!("⌨️  Typing: 'Hello, World!'");
    keyboard.type_string("Hello, World!")?;
    sleep(Duration::from_secs(1));
    
    // Type with interval
    println!("⌨️  Typing with 0.5 second intervals: 'Slow typing...'");
    keyboard.type_string_with_interval("Slow typing...", 0.5)?;
    sleep(Duration::from_millis(500));
    
    // Press Enter to go to next line
    println!("⌨️  Pressing Enter");
    keyboard.press_key("enter")?;
    sleep(Duration::from_millis(500));
    
    Ok(())
}

fn demo_key_presses(keyboard: &mut Keyboard) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔑 SECTION 2: Key Presses");
    println!("----------------------------------------");
    
    // Test various keys
    let test_keys = ["tab", "space", "backspace", "delete"];
    
    for key in test_keys {
        println!("⌨️  Pressing key: {}", key);
        keyboard.press_key(key)?;
        sleep(Duration::from_millis(300));
    }
    
    // Press Enter again
    keyboard.press_key("enter")?;
    sleep(Duration::from_millis(500));
    
    // Test arrow keys
    println!("⌨️  Testing arrow keys (up, down, left, right)");
    let arrow_keys = ["up", "down", "left", "right"];
    for key in arrow_keys {
        keyboard.press_key(key)?;
        sleep(Duration::from_millis(200));
    }
    
    // Test function keys
    println!("⌨️  Testing function keys (F1, F2, F3)");
    let function_keys = ["f1", "f2", "f3"];
    for key in function_keys {
        keyboard.press_key(key)?;
        sleep(Duration::from_millis(200));
    }
    
    Ok(())
}

fn demo_hotkeys(keyboard: &mut Keyboard) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔥 SECTION 3: Hotkey Combinations");
    println!("----------------------------------------");
    
    // Test common hotkeys
    println!("⌨️  Testing Ctrl+A (Select All)");
    keyboard.press_hotkey(&["ctrl", "a"])?;
    sleep(Duration::from_millis(500));
    
    println!("⌨️  Testing Ctrl+C (Copy)");
    keyboard.press_hotkey(&["ctrl", "c"])?;
    sleep(Duration::from_millis(500));
    
    println!("⌨️  Testing Ctrl+V (Paste)");
    keyboard.press_hotkey(&["ctrl", "v"])?;
    sleep(Duration::from_millis(500));
    
    println!("⌨️  Testing Ctrl+Z (Undo)");
    keyboard.press_hotkey(&["ctrl", "z"])?;
    sleep(Duration::from_millis(500));
    
    // Test Alt+Tab (switch windows)
    println!("⌨️  Testing Alt+Tab (switch windows)");
    keyboard.press_hotkey(&["alt", "tab"])?;
    sleep(Duration::from_secs(1));
    
    Ok(())
}

fn demo_advanced_patterns(keyboard: &mut Keyboard) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 SECTION 4: Advanced Patterns");
    println!("----------------------------------------");
    
    // Type a formatted text
    println!("⌨️  Typing formatted text with special characters");
    keyboard.type_string("Special characters: !@#$%^&*()_+-=[]{}|;':\",./<>?`~")?;
    sleep(Duration::from_millis(500));
    
    keyboard.press_key("enter")?;
    sleep(Duration::from_millis(300));
    
    // Multiple key presses
    println!("⌨️  Pressing 'a' key 5 times with 0.2 second intervals");
    keyboard.press_key_multiple("a", 5, 0.2)?;
    sleep(Duration::from_millis(500));
    
    keyboard.press_key("enter")?;
    sleep(Duration::from_millis(300));
    
    // Complex hotkey combination
    println!("⌨️  Testing Ctrl+Shift+A (complex hotkey)");
    keyboard.press_hotkey(&["ctrl", "shift", "a"])?;
    sleep(Duration::from_millis(500));
    
    Ok(())
}

fn countdown(seconds: u32) {
    for i in (1..=seconds).rev() {
        print!("{}... ", i);
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
    println!("Go!");
}
