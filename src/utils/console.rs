/// Clear the console and move the cursor to the top left corner
pub fn console_clear() {
    println!("\x1B[2J\x1B[1;1H");
}
