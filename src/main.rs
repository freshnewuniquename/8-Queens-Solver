use std::{
    env,
    fs::File,
    io::{stdout, Read, Write},
    path::{Component, Path},
};
mod board;
mod board_builder;
mod search;

#[allow(dead_code)]
fn interactive_menu() {
    todo!();
}

/// A function that takes a file path, and reads the file data to the data buffer provided.
///
/// The number of bytes read will be returned.
/// If the value is 0, then the file was not read successfully.
fn read_file_to(file_path: String, data: &mut [u8]) -> usize {
    println!("Received {file_path}");

    match File::open(&file_path) {
        Ok(mut file_handle) => {
            let res = file_handle.read(data);
            match res {
                Ok(read) => read,
                Err(desc) => {
                    println!("\"{file_path}\" can't be read. [{desc}]");
                    return 0;
                }
            }
        }
        Err(desc) => {
            // There are some quirks with the let-else statement (The return thing), so not using it rn.
            let Some(file_name) = Path::new(&file_path).file_name() else {
                println!("No file name found.");
                return 0;
            };
            let file_name = file_name.to_str().expect("Not a valid UTF-8 file name.");

            println!("\"{file_path}\" can't be opened, or does not exist. [{desc}]");

            if !Path::new(&file_path).components().any(|x| {
                if let Component::Normal(x) = x {
                    x == "src"
                } else {
                    false
                }
            }) {
                let src_path = Path::new("./src").join(file_name);
                let src_path = src_path.to_string_lossy();

                println!("Searching for \"{src_path}\".");
                read_file_to(src_path.to_string(), data)
            } else {
                0
            }
        }
    }
}

fn main() {
    let mut cli_options = env::args_os();
    let mut file_buffer = [0; 128 * 128]; // Supports up to 128-Queens. But only 26 addressable squares using CSV.

    if let Err(_) = writeln!(stdout(), "") {
        // Is stdout accessible?
        // Terminate program if not accessible.
        return;
    }

    // TODO: Maybe allow more than one board per run.
    let mut trustable = false;
    let mut init_data = (0, 0);
    let mut goal_data = (0, 0);
    let mut files_read_count = 0;
    let mut read = 0;

    if cli_options.len() <= 1 {
        let init = read_file_to("init".into(), &mut file_buffer);
        let goal = read_file_to("goal".into(), &mut file_buffer[init..]);

        init_data = (0, init);
        goal_data = (init, goal);
    } else {
        let exec_name = cli_options.next().unwrap_or_default();
        let exec_name = exec_name.to_string_lossy();
        let exec_name = exec_name.rsplit_once('/').unwrap_or(("", &exec_name)).1;

        for option in cli_options {
            if let Some(option) = option.to_str() {
                if option.starts_with('-') {
                    let option = option.split_once('=').unwrap_or((option, ""));

                    match (option.0, option.1) {
                        ("--help" | "-h", _) => {
                            println!(
                                "Usage: {exec_name} [OPTIONS] INPUT\n{}",
                                concat!(
                                    "Solves a N-Queen puzzle from the given input.\n\n",
                                    "Options:\n",
                                    "  -h, --help\t\tDisplays this message.\n",
                                    "      --trust\t\tRead the following input file without performing any checks (Not recommended). ")
                            );
                        }
                        ("--trust", _) => {
                            // Trust only works on one file.
                            trustable = true;
                        }
                        _ => {
                            println!("{exec_name}: invalid option '{}'\nTry '{exec_name} --help' for more information.", option.0);
                        }
                    }
                    continue;
                }

                let read_new = read_file_to(option.into(), &mut file_buffer[read..]).clone();
                if read_new == read {
                    continue;
                }

                if files_read_count % 2 == 0 {
                    init_data = (read, read_new);
                } else {
                    goal_data = (read, read_new);
                }

                read = read_new;
                files_read_count += 1;
            } else {
                println!(
                    "\"{}\" is not a valid UTF-8 argument. Command ignored, proceeding...",
                    option.to_string_lossy()
                );
            }
        }
    }
    let init_data = if init_data != (0, 0) {
        unsafe { std::str::from_utf8_unchecked(&file_buffer[init_data.0..init_data.1]) }
    } else {
        ""
    };
    let goal_data = if goal_data != (0, 0) {
        unsafe { std::str::from_utf8_unchecked(&file_buffer[goal_data.0..goal_data.1]) }
    } else {
        ""
    };

    // TODO: allow user to set the board size.
    let board = board_builder::BoardBuilder::<8>::new()
        .trust(trustable)
        .pipe_if(!init_data.is_empty(), |s| s.set_init(init_data))
        .pipe_if(!goal_data.is_empty(), |s| s.set_init(goal_data))
        // .set_init(unsafe { std::str::from_utf8_unchecked(&file_data[..read]) })
        .build();

    let mut board = match board {
        Ok(x) => x,
        Err(msg) => {
            println!("{msg}");
            return;
        }
    };

    println!("{board}");
    let moves = board.solve();
    board.print_moves(&moves);
}
