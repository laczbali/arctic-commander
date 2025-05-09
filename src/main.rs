mod utils;
use utils::console::print_local_state;

mod states;
use states::local::LocalState;
//use states::remote::RemoteState;

fn main() {
    utils::console::console_clear();
    let local_state = LocalState::new();

    loop {
        print_local_state(&local_state);

        let input = utils::console::get_input();

        match input.as_str() {
            "ls" => {
                list_dir(&local_state);
            }

            "exit" => {
                break;
            }

            _ => {
                println!("Unkown command [{}], type \"help\" for options", input);
            }
        }
    }
}

fn list_dir(local_state: &LocalState) {
    let files = utils::filesystem::list_dir(&local_state.working_dir);
    let mut file_index = 0;
    for file in files {
        let relative_path = file.strip_prefix(&local_state.working_dir).unwrap_or(&file);
        let dir_indicator = if file.is_dir() { "/" } else { "" };
        println!("[{}] {}", &file_index, relative_path.display());
        file_index += 1;
    }
}
