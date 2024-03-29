use std::{
    env,
    fs::File,
    io::{stdout, Read, Write},
    path::{Component, Path},
    time::Instant,
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
    eprintln!("Received {file_path}");

    match File::open(&file_path) {
        Ok(mut file_handle) => {
            let res = file_handle.read(data);
            match res {
                Ok(read) => read,
                Err(desc) => {
                    eprintln!("\"{file_path}\" can't be read. [{desc}]");
                    return 0;
                }
            }
        }
        Err(desc) => {
            // There are some quirks with the let-else statement (The return thing), so not using it rn.
            let Some(file_name) = Path::new(&file_path).file_name() else {
                eprintln!("No file name found.");
                return 0;
            };
            let file_name = file_name.to_str().expect("Not a valid UTF-8 file name.");

            eprintln!("\"{file_path}\" can't be opened, or does not exist. [{desc}]");

            if !Path::new(&file_path).components().any(|x| {
                if let Component::Normal(x) = x {
                    x == "states"
                } else {
                    false
                }
            }) {
                // Search relative to project root.
                let src_path = Path::new("./src/states").join(file_name);
                let src_path = src_path.to_string_lossy();

                let read_len = read_file_to(src_path.to_string(), data);

                if read_len != 0 {
                    return read_len;
                }

                // Search relative to ./src
                let src_path = Path::new("./states").join(file_name);
                let src_path = src_path.to_string_lossy();

                let read_len = read_file_to(src_path.to_string(), data);

                if read_len != 0 {
                    return read_len;
                }

                // Search relative to ./target/$MODE/
                let src_path = Path::new("../../src/states").join(file_name);
                let src_path = src_path.to_string_lossy();

                read_file_to(src_path.to_string(), data)
            } else {
                0
            }
        }
    }
}

fn main() {
    let execution_time = std::time::Instant::now();

    const N: usize = 8;
    let mut cli_options = env::args_os();
    let mut file_buffer = [0; 128 * 128]; // Supports up to 128-Queens. But only 26 addressable squares using CSV.

    if let Err(_) = writeln!(stdout(), "") {
        // Is stdout accessible?
        // Terminate program if not accessible.
        return;
    }

    // TODO: Maybe allow more than one board per run.
    let mut trustable = false;
    let mut init_range = (0, 0);
    let mut goal_range = (0, 0);
    let mut files_read_count = 0;
    let mut files_tried_count = 0;
    let mut read = 0;
    let mut quiet = false;
    let mut benchmark = cfg!(debug_assertions);
    let mut terminate = false;

    let exec_name = cli_options.next().unwrap_or_default();
    let exec_name = exec_name.to_string_lossy();
    let exec_name = exec_name.rsplit_once('/').unwrap_or(("", &exec_name)).1;

    for option in cli_options {
        if let Some(option) = option.to_str() {
            if option.starts_with('-') {
                let option = option.split_once('=').unwrap_or((option, ""));

                // TODO: Adds support for the grouping of small options.
                match (option.0, option.1) {
                    ("-b" | "--bench", _) => {
                        benchmark = true;
                    }
                    ("-h" | "--help", _) => {
                        println!(
                            "Usage: {exec_name} [OPTIONS] INPUT\n{}",
                            concat!(
                                "Solves a N-Queen puzzle from the given input.\n\n",
                                "Options:\n",
                                "  -b,  --bench\t\tDisplays the running time for some parts of the program.\n",
                                "  -h, --help\t\tDisplays this message.\n",
                                "      --trust\t\tRead the following input file without performing any checks (Not recommended).\n",
                                "  -q, --quiet\t\tSupresses the program output."
                            )
                        );
                        terminate = true;
                    }
                    ("--trust", _) => {
                        trustable = true;
                    }
                    ("-q" | "--quiet", _) => {
                        quiet = true;
                    }
                    _ => {
                        println!("{exec_name}: invalid option '{}'\nTry '{exec_name} --help' for more information.", option.0);
                    }
                }
                continue;
            }

            files_tried_count += 1;

            let read_new = read + read_file_to(option.into(), &mut file_buffer[read..]);
            if read_new == read {
                continue;
            }

            if files_read_count % 2 == 0 {
                init_range = (read, read_new);
            } else {
                goal_range = (read, read_new);
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

    if terminate {
        return;
    }

    if files_tried_count == 0 {
        let init = read_file_to("init".into(), &mut file_buffer);
        if init != 0 {
            init_range = (0, init);
            files_read_count += 1;
            files_tried_count += 1;
        }
    }

    if files_tried_count == 1 {
        let goal = read_file_to("goal".into(), &mut file_buffer[init_range.1..]);
        if goal != 0 {
            goal_range = (init_range.1, goal);
            files_read_count += 1;
            files_tried_count += 1;
        }
    }

    // Panics if found cannot be read.
    if init_range == (0, 0) {
        panic!("Init file not found or cannot be read.");
    } else if goal_range == (0, 0) {
        panic!("Goal file not found or cannot be read.");
    }

    // Convert bytes to strings.
    let init_data = if init_range != (0, 0) {
        unsafe { std::str::from_utf8_unchecked(&file_buffer[init_range.0..init_range.1]) }
    } else {
        ""
    };
    let goal_data = if goal_range != (0, 0) {
        unsafe { std::str::from_utf8_unchecked(&file_buffer[goal_range.0..goal_range.1]) }
    } else {
        ""
    };

    let start = Instant::now();

    // TODO: allow user to set the board size.
    let board = board_builder::BoardBuilder::<N>::new()
        .trust(trustable)
        .pipe_if(!init_data.is_empty(), |s| s.set_init(init_data))
        .pipe_if(!goal_data.is_empty(), |s| s.set_goal(goal_data))
        .build();

    let mut board = match board {
        Ok(x) => x,
        Err(msg) => {
            if !quiet {
                println!("{msg}");
            }
            return;
        }
    };

    if benchmark && !quiet {
        let elapsed = start.elapsed();
        println!(
            "\nTime used for input reading: {}ms ({}μs)",
            elapsed.as_millis(),
            elapsed.as_micros()
        );
    }

    let start = std::time::Instant::now();

    let moves = board.solve();

    if benchmark && !quiet {
        let elapsed = start.elapsed();
        println!(
            "\nTime used for solve(): {}ms ({}μs)",
            elapsed.as_millis(),
            elapsed.as_micros()
        );
    }

    if !quiet {
        let start = std::time::Instant::now();

        println!("{board}\n\nInitial state\n\n");
        board.replay_moves(&moves);

        if benchmark {
            let elapsed = start.elapsed();
            println!(
                "\nTime used for replaying moves: {}ms ({}μs)",
                elapsed.as_millis(),
                elapsed.as_micros()
            );
        }

        if cfg!(target_os = "windows") {
            println!("Press <Enter> to continue.");
            std::io::stdin().read_line(&mut String::new()).unwrap();
        }
    }

    if moves.len() > 1 {
        println!("A solution with {} moves found.", moves.len());
    } else {
        println!("A solution with {} move found.", moves.len());
    }

    let elapsed = execution_time.elapsed();
    println!(
        "Time taken: {:.6} seconds ({}μs)",
        elapsed.as_micros() as f64 / 1000. / 1000.,
        elapsed.as_micros()
    );
}
