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
    let mut file_data = [0; 128 * 128]; // Supports up to 128-Queens. But only 26 addressable squares using CSV.

    if let Err(_) = writeln!(stdout(), "") {
        // Is stdout accessible?
        // Terminate program if not accessible.
        return;
    }

    if cli_options.len() <= 1 {
        let read = read_file_to("init".into(), &mut file_data);
        let mut board = board_builder::BoardBuilder::<8>::new()
            .trust(false)
            .set(unsafe { std::str::from_utf8_unchecked(&file_data[..read]) })
            .data_type(board_builder::InputDataType::CSV)
            .build()
            .unwrap();

        println!("{board}");
        println!("Valid: {}\n", board.validate_game());

        let moves = board.solve();
        println!("{}", board);
        println!("Valid: {}", board.validate_game());
        println!("Total moves: {}", moves);
    } else {
        let mut trustable = false;
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

                let read = read_file_to(option.into(), &mut file_data);
                if read == 0 {
                    continue;
                }

                // TODO: allow user to set the board size.
                let board = board_builder::BoardBuilder::<8>::new()
                    .trust(trustable)
                    .set(unsafe { std::str::from_utf8_unchecked(&file_data[..read]) })
                    .build();

                let mut board = match board {
                    Ok(x) => x,
                    Err(msg) => {
                        println!("{msg}");
                        continue;
                    }
                };
                trustable = false;

                println!("{board}");
                println!("Valid: {}\n", board.validate_game());

                let moves = board.solve();
                println!("{}", board);
                println!("Valid: {}", board.validate_game());
                println!("Total moves: {}", moves);
            } else {
                println!(
                    "\"{}\" is not a valid UTF-8 argument. Command ignored, proceeding...",
                    option.to_string_lossy()
                );
            }
        }
    }
}
