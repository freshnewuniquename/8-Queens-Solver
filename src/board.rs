#![allow(dead_code)]
use crate::search::{self, Search};
use std::io::{stdout, Write};

pub struct Board<const N: usize> {
    pub(super) init_state: [[u8; N]; N],
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Coord {
    pub row: i8,
    pub col: i8,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: deal with board size > 'z'.
        write!(f, "{}{}", (self.col as u8 + b'a') as char, self.row + 1)
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::convert::From<&str> for Coord {
    fn from(value: &str) -> Self {
        let bytes = value.as_bytes();
        Coord {
            row: value[1..].parse::<i8>().unwrap() - 1,
            col: (bytes[0] - b'a') as i8,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum BoardPrint {
    Empty = 0,
    Q,
    Pound,         // Move from
    VerticalSlash, // Move path
    BackwardSlash, // Move path
    ForwardSlash,  // Move path
    Hyphen,        // Move path
}

impl BoardPrint {
    fn new(id: u8) -> Self {
        id.into()
    }
    fn to_unicode_u8(self) -> u8 {
        let c: char = self.into();
        c as u8
    }
}

impl From<u8> for BoardPrint {
    fn from(value: u8) -> Self {
        use BoardPrint::*;
        match value {
            0 => Empty,
            1 => Q,
            2 => Pound,
            3 => VerticalSlash,
            4 => BackwardSlash,
            5 => ForwardSlash,
            6 => Hyphen,
            _ => todo!("Unknown symbol."),
        }
    }
}

impl From<BoardPrint> for char {
    fn from(value: BoardPrint) -> Self {
        use BoardPrint::*;
        match value {
            Empty => ' ',
            Q => 'Q',
            Pound => '#',
            VerticalSlash => '|',
            BackwardSlash => '\\',
            ForwardSlash => '/',
            Hyphen => '-',
        }
    }
}

impl std::fmt::Display for BoardPrint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = (*self).into();
        write!(f, "{}", c)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[allow(dead_code)]
pub enum Moves {
    Horizontal(Coord, Coord),
    Vertical(Coord, Coord),
    Diagonal(Coord, Coord),
    ThreeMoves1(Coord, Coord),
    ThreeMoves2(Coord, Coord),
    ThreeMoves3(Coord, Coord),
    NoPossibleMoves,
    Left(Coord, Coord),
    Right(Coord, Coord),
    Up(Coord, Coord),
    Down(Coord, Coord),
    UpLeft(Coord, Coord),
    UpRight(Coord, Coord),
    DownLeft(Coord, Coord),
    DownRight(Coord, Coord),
}

#[allow(dead_code)]
impl Moves {
    pub fn get_values(self) -> Option<(Coord, Coord)> {
        use Moves::*;
        match self {
            // TODO: Look for a way to solve this mess.
            Horizontal(src, dest)
            | Vertical(src, dest)
            | Diagonal(src, dest)
            | ThreeMoves1(src, dest)
            | ThreeMoves2(src, dest)
            | ThreeMoves3(src, dest)
            | Left(src, dest)
            | Right(src, dest)
            | Up(src, dest)
            | Down(src, dest)
            | UpLeft(src, dest)
            | UpRight(src, dest)
            | DownLeft(src, dest)
            | DownRight(src, dest) => Some((src, dest)),
            NoPossibleMoves => None,
        }
    }
    pub fn get_specific_direction(self) -> Self {
        use Moves::*;

        if let Some((src, dest)) = self.get_values() {
            if src == dest {
                return NoPossibleMoves;
            }
        }

        match self {
            Horizontal(src, dest) => {
                if dest.col > src.col {
                    Right(src, dest)
                } else {
                    Left(src, dest)
                }
            }
            Vertical(src, dest) => {
                if dest.row > src.row {
                    Up(src, dest)
                } else {
                    Down(src, dest)
                }
            }
            Diagonal(src, dest) => {
                if dest.row > src.row {
                    if dest.col > src.col {
                        UpRight(src, dest)
                    } else {
                        UpLeft(src, dest)
                    }
                } else if dest.col > src.col {
                    DownRight(src, dest)
                } else {
                    DownLeft(src, dest)
                }
            }
            x @ _ => x,
        }
    }
    pub fn get_src(self) -> Option<Coord> {
        Some(self.get_values()?.0)
    }
    pub fn get_dest(self) -> Option<Coord> {
        Some(self.get_values()?.1)
    }
}

impl<const N: usize> Default for Board<N> {
    fn default() -> Self {
        Board {
            init_state: [[0; N]; N],
        }
    }
}

impl<const N: usize> Board<N> {
    #[must_use]
    /// The constructor for the Board struct.
    pub fn new(init_data: &str) -> Board<N> {
        let mut board = Board::default();

        #[cfg(debug_assertions)]
        {
            Board::set(init_data, &mut board.init_state).unwrap();
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            Board::fast_set(init_data, &mut board.init_state);
        }
        board
    }
    /// Takes in a string of data, and buffer, then automatically determine the type
    /// of data to decode, fill in into the buffer, and returns the result.
    ///
    /// # Errors
    ///
    /// This function will return an error if the input data does not match any of the
    /// current supported file types.
    pub fn set(data: &str, buf: &mut [[u8; N]; N]) -> Result<(), String> {
        if let Err(fen_desc) = Self::set_with_fen(data, buf) {
            if let Err(csv_desc) = Self::set_with_csv(data, buf) {
                return Err(format!(
                    "Unable to determine file data type.\n[FEN: {fen_desc}]\n[CSV: {csv_desc}]"
                ));
            }
        }
        Ok(())
    }
    /// Takes in a string of data, and buffer, then automatically determine the type
    /// of data to decode, fill in into the buffer.
    ///
    /// This is the unsafe version of [`set`]. For more information, refer to that
    /// function.
    #[inline(always)]
    pub unsafe fn fast_set(data: &str, buf: &mut [[u8; N]; N]) {
        if data.as_bytes()[2] == b',' {
            Board::fast_set_with_csv(data, buf);
        } else {
            Board::fast_set_with_fen(data, buf);
        }
    }
    /// Sets the board's state with CSV of the queens coordinates.
    ///
    /// # Errors
    ///
    /// This function will return an error if the CSV data is invalid.
    pub fn set_with_csv(csv_data: &str, buf: &mut [[u8; N]; N]) -> Result<(), String> {
        let mut it = csv_data.split(',');

        let mut cur_count = 0;
        while cur_count < N {
            let coord = it.next().ok_or(format!(
                "Expected {N} queens from input, only {cur_count} found."
            ))?;

            if coord.as_bytes().len() < 2 {
                return Err(format!("Malformed Queen {} coordinates.", cur_count + 1));
            }

            let col = coord.as_bytes()[0].saturating_sub(b'a' - 1);
            let row = coord.as_bytes()[1].saturating_sub(b'0');

            if col > N as u8 || row > N as u8 || col == 0 || row == 0 {
                return Err(format!(
                    "Invalid Queen {} coordinates value.",
                    cur_count + 1
                ));
            }
            buf[row as usize - 1][col as usize - 1] = 1;
            cur_count += 1;
        }
        Ok(())
    }
    /// Sets the board's state with CSV of the queens coordinates.
    ///
    /// This is the unsafe version [`set_with_csv`].
    /// This function does not perform any checks to determine the validity of the CSV.
    #[inline(always)]
    pub unsafe fn fast_set_with_csv(csv_data: &str, buf: &mut [[u8; N]; N]) {
        let mut idx = 0;
        let csv_bytes = csv_data.as_bytes();

        while idx < N * 3 - 1 {
            let file = csv_bytes[idx] - b'a';
            let rank = csv_bytes[idx + 1] - b'1';
            buf[rank as usize][file as usize] = 1;
            idx += 3; // Skips a comma too.
        }
    }
    /// Reads the provided FEN, and input the queens into $buf.
    ///
    /// NOTE: If there are more than $N queens, the function will only return an Err()
    ///       after all the queens are placed into the board.
    /// NOTE: The board will be left in an incomplete state when an error occurs, instead of
    ///       being left in an untouched state.
    pub fn set_with_fen(fen_data: &str, buf: &mut [[u8; N]; N]) -> Result<(), String> {
        // Splits the metadata from the board.
        let fen_data = fen_data.split_once(' ').unwrap_or((fen_data, "")).0;

        let ranks_total = fen_data
            .bytes()
            .fold(0, |acc, x| if x == b'/' { acc + 1 } else { acc });
        if ranks_total != N - 1 {
            return Err(format!("Expected {N} ranks, but found {}.", ranks_total+1));
        }

        let mut it = fen_data.split('/');
        let mut cur_rank = N;
        let mut total_queens = 0;

        while let Some(rank) = it.next() {
            let mut cur_file = 0u8;
            let mut last_digit_index = 0;
            let mut in_digit_range = false;

            for (i, x) in rank.bytes().enumerate() {
                let mut valid = true;

                if x.is_ascii_digit() {
                    if !in_digit_range {
                        in_digit_range = true;
                        last_digit_index = i;
                    }
                } else if x == b'q' || x == b'Q' {
                    if in_digit_range {
                        // Parse the number. Should be safe to call .unwrap().
                        cur_file += rank[last_digit_index..i].parse::<u8>().unwrap();
                        in_digit_range = false;
                    }

                    if cur_file < N as u8 {
                        buf[cur_rank - 1][cur_file as usize] = 1;
                    }
                    cur_file += 1;
                    total_queens += 1;
                } else {
                    valid = false;
                }

                if !valid {
                    return Err(format!(
                        "Unexpected token '{}' on rank {cur_rank}",
                        x as char,
                    ));
                }
            }

            if in_digit_range {
                cur_file += rank[last_digit_index..].parse::<u8>().unwrap();
            }

            if cur_file != N as u8 {
                return Err(format!(
                    "Expected a total of {N} files on rank {cur_rank}, found {cur_file}."
                ));
            }
            cur_rank -= 1;
        }

        if total_queens != N {
            return Err(format!(
                "Expected a total of {N} queens, {total_queens} found."
            ));
        }
        Ok(())
    }
    /// Reads the provided FEN, and input the queens into $buf.
    ///
    /// This is the unsafe version of [`set_with_fen`].
    /// This function does not perform any checks to determine the validity of the FEN.
    #[inline(always)]
    pub unsafe fn fast_set_with_fen(fen_data: &str, buf: &mut [[u8; N]; N]) {
        let mut raw_init_state: *mut u8 = &mut buf[0][0];
        let fen_data = fen_data.as_bytes();

        let mut idx = 0;
        while fen_data[idx] != b' ' {
            if fen_data[idx] & 0x60 != 0 {
                *raw_init_state = 1;
                raw_init_state = raw_init_state.add(1);
                idx += 1;
            } else {
                let mut n = fen_data[idx] - b'0';
                // XOR 0b0011_0000, to filter out non-digits.
                // 0 - 0b0011_0000
                // 9 - 0b0011_1001
                //   - 0b0010_0000
                // Q - 0b0101_0001
                while fen_data[idx + 1] ^ 0x30 > 0x10 {
                    n *= 10;
                    idx += 1;
                    n += fen_data[idx] - b'0';
                }
                raw_init_state = raw_init_state.add(n as usize);
            }
        }
    }
    /// Moves the selected chess piece to the given location, from the given chess coordinates notations.
    ///
    /// NOTE: This function does not check for the move validity, and will just move them regardless.
    ///
    /// # Examples:
    ///
    /// Basic usage:
    /// ```
    /// move_piece_with_coords("a2", "a4");
    /// ```
    fn move_piece_with_coords(&mut self, src: &str, dest: &str) {
        let src = src.as_bytes();
        let dest = dest.as_bytes();

        let s = ((src[0] - b'a') as usize, (src[1] - b'1') as usize);
        let d = ((dest[0] - b'a') as usize, (dest[1] - b'1') as usize);

        if self.init_state[s.1][s.0] == 1 {
            self.init_state[s.1][s.0] = 0;
            self.init_state[d.1][d.0] = 1;
        } else {
            println!("Move invalid");
        }
    }
    /// Returns an N-array of a row and column tuple of the queens position on the board.
    fn get_queens_pos(map: [[u8; N]; N]) -> [Coord; N] {
        let mut queens_pos = [Coord::default(); N];
        let mut idx = 0;

        for (row_n, row) in map.iter().enumerate().rev() {
            for (col_n, val) in row.iter().enumerate() {
                if *val >= 1 {
                    if idx == N {
                        break;
                    }
                    queens_pos[idx] = Coord {
                        row: row_n as i8,
                        col: col_n as i8,
                    };
                    idx += 1;
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            queens_pos.sort_unstable_by_key(|x| x.col);
            queens_pos.sort_by_key(|x| x.row);
        }
        queens_pos
    }
    #[inline(always)]
    pub fn solve(&mut self) -> Vec<Moves> {
        // let mut ds = <search::DFS<_> as Search>::with_capacity(64);
        // let mut ds = <search::BFS<_> as Search>::with_capacity(700_000);
        // let mut ds = <search::Dijkstra<_> as Search>::with_capacity(700_000);
        let mut ds = <search::AStar<_> as Search>::with_capacity(700_000);

        let map_list = Board::get_queens_pos(self.init_state);
        if Self::validate_list(map_list) {
            return vec![];
        }

        // const DIAGONALS: usize = N*2-1;
        const DIAGONALS: usize = 15;

        if DIAGONALS != N * 2 - 1 {
            todo!("Please change the $DIAGONALS value manually in the code, to use other board sizes.");
        }

        let mut lowest_solve = u16::MAX;
        let mut lowest_moves = vec![];

        #[allow(unused_mut)]
        let mut _nodes_generated = 1; // Including the root node.
        let mut _explored = 0;
        let mut _pruned = 0;
        let mut _max_frontier_len = 0;

        ds.push((map_list, 0, Vec::with_capacity(N)));

        let diag_rel_bot_left = |x: Coord| (x.col + x.row) as usize;
        let diag_rel_bot_right = |x: Coord| N - 1 + (-x.col + x.row) as usize;

        let get_counts = |map_list: [Coord; N]| {
            let mut col_count = [0; N];
            let mut row_count = [0; N];
            // let mut diag_backslash_count = [0; N*2-1];
            // let mut diag_fwdslash_count = [0; N*2-1];
            let mut diag_backslash_count = [0; DIAGONALS];
            let mut diag_fwdslash_count = [0; DIAGONALS];

            for x in map_list {
                unsafe {
                    *col_count.get_unchecked_mut(x.col as usize) += 1;
                    *row_count.get_unchecked_mut(x.row as usize) += 1;
                    *diag_backslash_count.get_unchecked_mut(diag_rel_bot_left(x)) += 1;
                    *diag_fwdslash_count.get_unchecked_mut(diag_rel_bot_right(x)) += 1;
                }
            }

            (
                col_count,
                row_count,
                diag_backslash_count,
                diag_fwdslash_count,
            )
        };

        // let calculate_heuristic = |col_count: [u8; N], row_count: [u8; N], diag_backslash_count: [u8; N*2-1], diag_fwdslash_count: [u8; N*2-1]| {
        let calculate_heuristic =
            |col_count: [u8; N],
             row_count: [u8; N],
             diag_backslash_count: [u8; DIAGONALS],
             diag_fwdslash_count: [u8; DIAGONALS]| {
                col_count
                    .into_iter()
                    .fold(0, |acc, x| acc + if x <= 1 { 0 } else { x - 1 })
                    + row_count
                        .into_iter()
                        .fold(0, |acc, x| acc + if x <= 1 { 0 } else { x - 1 })
                    + diag_backslash_count
                        .into_iter()
                        .fold(0, |acc, x| acc + if x <= 1 { 0 } else { x - 1 })
                    + diag_fwdslash_count
                        .into_iter()
                        .fold(0, |acc, x| acc + if x <= 1 { 0 } else { x - 1 })
            };

        'main: while let Some((map_list, total, moves)) = ds.pop_next() {
            let new_total = total + 1;
            if new_total >= lowest_solve {
                // Pruning.
                #[cfg(debug_assertions)]
                {
                    _pruned += 1;
                }
                continue;
            }
            #[cfg(debug_assertions)]
            {
                _explored += 1;
            }

            // Search the whole row.
            for queen_i in 0..N {
                let (cc, mut rc, mut dbc, mut dfc) = get_counts(map_list);

                dbc[diag_rel_bot_left(map_list[queen_i])] -= 1;
                dfc[diag_rel_bot_right(map_list[queen_i])] -= 1;
                rc[map_list[queen_i].row as usize] -= 1;

                let (mut row_i, max) = if cc[map_list[queen_i].col as usize] > 1 {
                    let y = map_list[queen_i];
                    let mut min = 0;
                    let mut max = N as i8;

                    for x in map_list {
                        if x.col == y.col {
                            if x.row < y.row && x.row > min {
                                min = x.row;
                            } else if x.row > y.row && x.row < max {
                                max = x.row;
                            }
                        }
                    }
                    (min, max)
                } else {
                    (0, N as i8)
                };

                while row_i < max {
                    let mut new_map_list = map_list;
                    let mut new_moves = moves.clone();
                    new_map_list[queen_i].row = row_i;

                    rc[row_i as usize] += 1;
                    dbc[(row_i + map_list[queen_i].col) as usize] += 1;
                    dfc[N - 1 + (row_i - map_list[queen_i].col) as usize] += 1;

                    let estimated_cost = calculate_heuristic(cc, rc, dbc, dfc) as usize;

                    rc[row_i as usize] -= 1;
                    dbc[(row_i + map_list[queen_i].col) as usize] -= 1;
                    dfc[N - 1 + (row_i - map_list[queen_i].col) as usize] -= 1;

                    new_moves.push(Moves::Vertical(
                        map_list[queen_i],
                        Coord {
                            row: row_i,
                            col: map_list[queen_i].col,
                        },
                    ));

                    row_i += 1;

                    if estimated_cost == 0 {
                        lowest_solve = new_total;
                        lowest_moves = new_moves;

                        if ds.is_abort_on_found() {
                            break 'main;
                        } else {
                            continue 'main;
                        }
                    }

                    ds.apply_node_heuristic(estimated_cost)
                        .apply_path_cost(new_total as usize)
                        .push((new_map_list, new_total, new_moves));
                    #[cfg(debug_assertions)]
                    {
                        _nodes_generated += 1;
                    }
                }
            }

            // Search the whole column.
            for queen_i in 0..N {
                let (mut cc, rc, mut dbc, mut dfc) = get_counts(map_list);
                dbc[diag_rel_bot_left(map_list[queen_i])] -= 1;
                dfc[diag_rel_bot_right(map_list[queen_i])] -= 1;
                cc[map_list[queen_i].col as usize] -= 1;

                let (mut col_i, max) = if rc[map_list[queen_i].row as usize] > 1 {
                    let y = map_list[queen_i];
                    let mut min = 0;
                    let mut max = N as i8;

                    for x in map_list {
                        if x.row == y.row {
                            if x.col < y.col && x.col > min {
                                min = x.col;
                            } else if x.col > y.col && x.col < max {
                                max = x.col;
                            }
                        }
                    }
                    (min, max)
                } else {
                    (0, N as i8)
                };

                while col_i < max {
                    let mut new_map_list = map_list;
                    let mut new_moves = moves.clone();
                    new_map_list[queen_i].col = col_i;

                    cc[col_i as usize] += 1;
                    dbc[(col_i + map_list[queen_i].row) as usize] += 1;
                    dfc[N - 1 + (-col_i + map_list[queen_i].row) as usize] += 1;

                    let estimated_cost = calculate_heuristic(cc, rc, dbc, dfc) as usize;

                    cc[col_i as usize] -= 1;
                    dbc[(col_i + map_list[queen_i].row) as usize] -= 1;
                    dfc[N - 1 + (-col_i + map_list[queen_i].row) as usize] -= 1;

                    new_moves.push(Moves::Horizontal(
                        map_list[queen_i],
                        Coord {
                            col: col_i,
                            row: map_list[queen_i].row,
                        },
                    ));

                    col_i += 1;

                    if estimated_cost == 0 {
                        lowest_solve = new_total;
                        lowest_moves = new_moves;

                        if ds.is_abort_on_found() {
                            break 'main;
                        } else {
                            continue 'main;
                        }
                    }

                    ds.apply_node_heuristic(estimated_cost)
                        .apply_path_cost(new_total as usize)
                        .push((new_map_list, new_total, new_moves));
                    #[cfg(debug_assertions)]
                    {
                        _nodes_generated += 1;
                    }
                }
            }

            // Search the whole diagonal (Relative to a1, a.k.a. backward slash).
            for queen_i in 0..N {
                let cur_q = map_list[queen_i];
                let (mut cc, mut rc, mut dbc, mut dfc) = get_counts(map_list);

                let n = diag_rel_bot_left(cur_q) as i8;
                cc[cur_q.col as usize] -= 1;
                rc[cur_q.row as usize] -= 1;
                dbc[n as usize] -= 1;
                dfc[diag_rel_bot_right(cur_q)] -= 1;

                let mut diag_i = if n < N as i8 {
                    Coord { row: 0, col: n }
                } else {
                    Coord {
                        row: n - N as i8,
                        col: N as i8 - 1,
                    }
                };
                let mut max_row = if n < N as i8 { n + 1 } else { N as i8 };

                if dbc[n as usize] > 0 {
                    for x in map_list {
                        if x.row != cur_q.row && x.row - cur_q.row == -x.col + cur_q.col {
                            if x.row < cur_q.row {
                                if x.row > diag_i.row {
                                    diag_i = x;
                                }
                            } else if x.row < max_row {
                                max_row = x.row;
                            }
                        }
                    }
                }

                while diag_i.row < max_row {
                    let mut new_map_list = map_list;
                    let mut new_moves = moves.clone();
                    new_map_list[queen_i] = diag_i;

                    rc[diag_i.row as usize] += 1;
                    cc[diag_i.col as usize] += 1;
                    dbc[diag_rel_bot_left(diag_i)] += 1;
                    dfc[diag_rel_bot_right(diag_i)] += 1;

                    let estimated_cost = calculate_heuristic(cc, rc, dbc, dfc) as usize;

                    rc[diag_i.row as usize] -= 1;
                    cc[diag_i.col as usize] -= 1;
                    dbc[diag_rel_bot_left(diag_i)] -= 1;
                    dfc[diag_rel_bot_right(diag_i)] -= 1;

                    new_moves.push(Moves::Diagonal(map_list[queen_i], diag_i));

                    diag_i.row += 1;
                    diag_i.col -= 1;

                    if estimated_cost == 0 {
                        lowest_solve = new_total;
                        lowest_moves = new_moves;

                        if ds.is_abort_on_found() {
                            break 'main;
                        } else {
                            continue 'main;
                        }
                    }

                    ds.apply_node_heuristic(estimated_cost)
                        .apply_path_cost(new_total as usize)
                        .push((new_map_list, new_total, new_moves));
                    #[cfg(debug_assertions)]
                    {
                        _nodes_generated += 1;
                    }
                }
            }

            // Search the whole diagonal (Relative to h1, a.k.a. forward slash).
            for queen_i in 0..N {
                let cur_q = map_list[queen_i];
                let (mut cc, mut rc, mut dbc, mut dfc) = get_counts(map_list);

                let n = diag_rel_bot_right(cur_q) as i8;
                cc[cur_q.col as usize] -= 1;
                rc[cur_q.row as usize] -= 1;
                dbc[diag_rel_bot_left(cur_q)] -= 1;
                dfc[n as usize] -= 1;

                let mut diag_i = if n < N as i8 {
                    Coord {
                        row: 0,
                        col: N as i8 - 1 - n,
                    }
                } else {
                    Coord {
                        row: n - N as i8 + 1,
                        col: 0,
                    }
                };
                let mut max_row = if n < N as i8 { n + 1 } else { N as i8 };

                if dfc[n as usize] > 0 {
                    for x in map_list {
                        if x.row != cur_q.row && x.row - cur_q.row == x.col - cur_q.col {
                            if x.row < cur_q.row {
                                if x.row > diag_i.row {
                                    diag_i = x;
                                }
                            } else if x.row < max_row {
                                max_row = x.row;
                            }
                        }
                    }
                }

                while diag_i.row < max_row {
                    let mut new_map_list = map_list;
                    let mut new_moves = moves.clone();
                    new_map_list[queen_i] = diag_i;

                    rc[diag_i.row as usize] += 1;
                    cc[diag_i.col as usize] += 1;
                    dbc[diag_rel_bot_left(diag_i)] += 1;
                    dfc[diag_rel_bot_right(diag_i)] += 1;

                    let estimated_cost = calculate_heuristic(cc, rc, dbc, dfc) as usize;

                    rc[diag_i.row as usize] -= 1;
                    cc[diag_i.col as usize] -= 1;
                    dbc[diag_rel_bot_left(diag_i)] -= 1;
                    dfc[diag_rel_bot_right(diag_i)] -= 1;

                    new_moves.push(Moves::Diagonal(map_list[queen_i], diag_i));

                    diag_i.row += 1;
                    diag_i.col += 1;

                    if estimated_cost == 0 {
                        lowest_solve = new_total;
                        lowest_moves = new_moves;

                        if ds.is_abort_on_found() {
                            break 'main;
                        } else {
                            continue 'main;
                        }
                    }

                    ds.apply_node_heuristic(estimated_cost)
                        .apply_path_cost(new_total as usize)
                        .push((new_map_list, new_total, new_moves));
                    #[cfg(debug_assertions)]
                    {
                        _nodes_generated += 1;
                    }
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            _max_frontier_len = ds.len();
        }

        #[cfg(debug_assertions)]
        {
            ds = Search::new();
            dbg!(ds, _nodes_generated, _explored, _pruned, _max_frontier_len);
        }

        lowest_moves
    }
    #[inline(always)]
    fn validate_list(queens_pos: [Coord; N]) -> bool {
        let mut row_lookup = [false; N];
        for x in queens_pos {
            unsafe {
                if *row_lookup.get_unchecked(x.row as usize) {
                    return false;
                } else {
                    *row_lookup.get_unchecked_mut(x.row as usize) = true;
                }
            }
        }

        let mut col_lookup = [false; N];
        for x in queens_pos {
            unsafe {
                if *col_lookup.get_unchecked(x.col as usize) {
                    return false;
                } else {
                    *col_lookup.get_unchecked_mut(x.col as usize) = true;
                }
            }
        }

        let mut i = 0;
        while i < N {
            let mut ii = i + 1;
            while ii < N {
                if queens_pos[i].row.abs_diff(queens_pos[ii].row)
                    == queens_pos[i].col.abs_diff(queens_pos[ii].col)
                {
                    return false;
                }
                ii += 1;
            }
            i += 1;
        }
        true
    }
    pub fn validate_game(&mut self) -> bool {
        let mut queens_pos = Vec::with_capacity(N);

        // Mark the location of the queens, and check if more than one are on the same row.
        for (row_n, row) in self.init_state.iter().enumerate() {
            let mut has_queen = false;
            for (col_n, val) in row.iter().enumerate() {
                if val == &1 {
                    if has_queen {
                        return false;
                    } else {
                        has_queen = true;
                        queens_pos.push(Coord {
                            row: row_n as i8,
                            col: col_n as i8,
                        });
                    }
                }
            }
        }

        // Check if there are more than one queens on the same column.
        let mut col_lookup = [false; N];
        for x in &queens_pos {
            if col_lookup[x.col as usize] {
                return false;
            } else {
                col_lookup[x.col as usize] = true;
            }
        }

        // Check diagonally - O(N^2)
        let mut i = 0;
        while i < N {
            let mut ii = i + 1;
            while ii < N {
                if queens_pos[i].row.abs_diff(queens_pos[ii].row)
                    == queens_pos[i].col.abs_diff(queens_pos[ii].col)
                {
                    return false;
                }
                ii += 1;
            }
            i += 1;
        }

        true
    }
    pub fn replay_moves(&mut self, moves: &Vec<Moves>) {
        let mut map = self.init_state;
        let stdout = stdout();
        let mut stdout = stdout.lock();

        for (i, x) in moves.iter().enumerate() {
            let mut new_map = map;
            if let Some((src, dest)) = x.get_values() {
                map[src.row as usize][src.col as usize] = 0;
                map[dest.row as usize][dest.col as usize] = 1;
                new_map = map;

                new_map[src.row as usize][src.col as usize] = BoardPrint::Pound as u8;
                new_map[dest.row as usize][dest.col as usize] = BoardPrint::Q as u8;

                let (min, max) = if src > dest { (dest, src) } else { (src, dest) };

                match x {
                    Moves::Diagonal(_, _) => {
                        let mut src = src;
                        if src.row < dest.row {
                            src.row += 1;
                        } else {
                            src.row -= 1;
                        }

                        if src.col < dest.col {
                            src.col += 1;
                        } else {
                            src.col -= 1;
                        }

                        while src.row != dest.row {
                            new_map[src.row as usize][src.col as usize] =
                                if (src.row > dest.row) == (src.col > dest.col) {
                                    BoardPrint::ForwardSlash as u8
                                } else {
                                    BoardPrint::BackwardSlash as u8
                                };
                            if src.row < dest.row {
                                src.row += 1;
                            } else {
                                src.row -= 1;
                            }

                            if src.col < dest.col {
                                src.col += 1;
                            } else {
                                src.col -= 1;
                            }
                        }
                    }
                    Moves::Vertical(_, _) => {
                        for y in min.row + 1..max.row {
                            new_map[y as usize][src.col as usize] = BoardPrint::VerticalSlash as u8;
                        }
                    }
                    Moves::Horizontal(_, _) => {
                        for x in min.col + 1..max.col {
                            new_map[src.row as usize][x as usize] = BoardPrint::Hyphen as u8;
                        }
                    }
                    _ => {}
                }
            }

            writeln!(stdout, "{}\n", Self::to_string_inner(new_map)).unwrap();
            writeln!(
                stdout,
                "Move {}: {:?}\n\n",
                i + 1,
                x.get_specific_direction()
            )
            .unwrap();
        }
    }
    pub fn to_string(&self) -> String {
        Self::to_string_inner(self.init_state)
    }
    pub fn to_string_inner(map: [[u8; N]; N]) -> String {
        // TODO: const generate the $layout.
        // Using macro temporarily to store constants and stuff, as generics parameter can't be used with constant calculation as of yet.
        // Also acts as a central place to change all the constants, as the board output may not be final.
        macro_rules! cal {
            ($row: expr, $col: expr) => {
                ($row*2 + 1)*cal!(row_len) + $col*2 + 3
            };
            (is_newline $pos: expr, $cur_row: expr) => {
                // XXX: Assumes the pointer is aligned with the board border.
                // The row is also requested because of the variation of the row length and the board row length.
                $pos - $cur_row as usize*cal!(row_len) == cal!(row_len)-2
            };
            (is_intersection $pos: expr, $cur_row: expr) => {{
                let col = $pos - $cur_row as usize*cal!(row_len);
                $cur_row != 0 && $cur_row != N*2 && col != 2 && col != cal!(row_len)-2
            }};
            (row_len_only_board) => {
                N*2+1
            };
            (row_len) => {
                // The rank digit, a space, and a newline.
                cal!(row_len_only_board) + 3
            };
            (whole_len_no_file_indicators) => {
                cal!(row_len)*(N*2+1)
            };
            (whole_len) => {
                cal!(row_len)*(N*2+2)
            }
        }

        let mut layout = vec![b' '; cal!(whole_len)];

        for y in 0..N {
            for x in ((y + 1) % 2..N).step_by(2) {
                layout[cal!(y, x)] = b'.';
            }
        }

        let mut i = 2;
        let mut cur_row = 0;
        while i < cal!(whole_len_no_file_indicators) {
            if cal!(is_newline i, cur_row) {
                if cur_row & 1 == 0 {
                    layout[i] = b'-';
                    if cur_row != N * 2 {
                        layout[i + 2] = b'0' + N as u8 - cur_row as u8 / 2;
                    }
                } else {
                    layout[i] = b'|';
                }

                layout[i + 1] = b'\n';
                cur_row += 1;
                i += 2;
            } else if cur_row & 1 == 0 {
                if cal!(is_intersection i, cur_row) {
                    layout[i] = b'+';
                } else {
                    layout[i] = b'-';
                }

                layout[i + 1] = b'-';
            } else {
                layout[i] = b'|';
            }

            i += 2;
        }

        i += 1;
        let mut char = b'a';
        while i < cal!(whole_len_no_file_indicators) + cal!(row_len_only_board) + 1 {
            layout[i] = char;
            char += 1;
            i += 2;
        }

        for (row_n, row) in map[..].iter().rev().enumerate() {
            for (col_n, val) in row.iter().enumerate() {
                if *val >= 1 {
                    layout[cal!(row_n, col_n)] = BoardPrint::new(*val).to_unicode_u8();
                }
            }
        }

        // Guaranteed to be valid UTF-8, since only ASCII characters are being applied.
        return unsafe { String::from_utf8_unchecked(layout) };
    }
}

impl<const N: usize> std::fmt::Display for Board<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
