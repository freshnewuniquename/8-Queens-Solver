use std::{
    env,
    fs::File,
    io::{stdout, Read, Write},
};
mod board;

#[allow(dead_code)]
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
    let mut file_data = [0; 128 * 128]; // Supports up to 127-Queens.

    if let Err(_) = writeln!(stdout(), "") {
        // Is stdout accessible?
        // Terminate program if not accessible.
        return;
    }
    // Trying out other smaller board sizes.
    // let mut board = <board::Board::<4>>::new("a2,b2,c2,d2", "a1,a1,a1,a1");
    // board.solve();
    // return;
    if cli_options.len() <= 1 {
        // interactive_menu();
        read_file_to("src/init", &mut file_data);
        let mut board = <board::Board<8> as board::EightQueen>::new(unsafe {
            std::str::from_utf8_unchecked(&file_data)
        });
        println!("{board}");
        let ans = board.solve();
        println!("moves: {:?}", ans);
        board.print_moves(&ans);
    } else {
        for option in cli_options.skip(1) {
            if let Some(file_name) = option.to_str() {
                if !read_file_to(file_name, &mut file_data) {
                    continue;
                }
                let mut board = <board::Board<8> as board::EightQueen>::new(unsafe {
                    std::str::from_utf8_unchecked(&file_data)
                });
                println!("{board}");
                let ans = board.solve();
                println!("moves: {:?}", ans);
                board.print_moves(&ans);
            } else {
                println!(
                    "\"{}\" is not a valid file name. File ignored, proceeding...",
                    option.to_string_lossy()
                );
            }
        }
    }
}
