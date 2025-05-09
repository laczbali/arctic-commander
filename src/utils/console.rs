use std::io::Write;

pub fn print_local_state(local_state: &crate::states::local::LocalState) {
    println!("");

    if local_state.selected_files.len() != 0 {
        println!("Selected files");
    }
    for file in &local_state.selected_files {
        println!("- {}", file.display());
    }

    println!("@ {}", local_state.working_dir.display());
}

pub fn get_input() -> String {
    print!("> ");
    std::io::stdout().flush().unwrap_or(());

    let mut input = String::new();
    _ = std::io::stdin().read_line(&mut input).unwrap_or(0);
    return input.trim().to_string();
}

/// Clear the console and move the cursor to the top left corner
pub fn console_clear() {
    println!("\x1B[2J\x1B[1;1H");
}
