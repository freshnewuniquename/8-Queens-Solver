use std::{env, fs::File, io::{Read, stdout, Write}};
mod board;

fn interactive_menu() {
    todo!();
}

fn read_file_to(file_name: &str, data: &mut [u8]) -> bool {
    println!("received {file_name}");

    if let Ok(mut file_handle) = File::open(file_name) {
        if let Err(_) = file_handle.read(data) {
            println!("\"{file_name}\" can't be read.");
            return false;
        }
        return true;
    } else {
        println!("\"{file_name}\" can't be opened, or does not exist.");
        return false;
    }
}

fn main() {
    let cli_options = env::args_os();
    let mut file_data = [0; 128*128]; // Supports up to 128-Queens.

    if let Err(_) = writeln!(stdout(), "") { // Is stdout accessible?
        // Terminate program if not accessible.
        return;
    }

    if cli_options.len() <= 1 {
        // interactive_menu();
        read_file_to("src/init", &mut file_data);
        let mut board = board::Board::<8>::create(unsafe { std::str::from_utf8_unchecked(&file_data) });
        loop {
            println!("{board}");
            println!("valid={}", board.validate_game());
            println!("moves: {}", board.solve());
            println!("{}", board);
            break;
            
            //let mut str1 = String::from("   ");
            //let mut str2 = String::from("   ");
            //unsafe {
            //    std::io::stdin().read(str1.as_bytes_mut()).unwrap();
            //    std::io::stdin().read(str2.as_bytes_mut()).unwrap();
            //}
            //board.move_piece(&str1, &str2);
        }
    } else {
        for option in cli_options.skip(1) {
            if let Some(file_name) = option.to_str() {
                if !read_file_to(file_name, &mut file_data) {
                    continue;
                }
                let board = board::Board::<8>::create(unsafe { std::str::from_utf8_unchecked(&file_data) });
                println!("{board}");
            } else {
                println!("\"{}\" is not a valid file name. File ignored, proceeding...", option.to_string_lossy());
            }
        }
    }
}