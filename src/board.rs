#[allow(dead_code)]
pub struct Board<const N: usize> {
    cur_state: [[u8; N]; N],
    goal_state: [[u8; N]; N],
    moves: Vec<String>,
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
struct Coord {
    row: i8,
    col: i8,
}

pub trait EightQueen {
    fn new(csv_init_data: &str) -> Board<8>;
    fn set_with_csv(&mut self, csv_init_data: &str);
    fn fast_set_with_csv(&mut self, csv_init_data: &str);
}

impl EightQueen for Board<8> {
    fn new(csv_init_data: &str) -> Board<8> {
        let mut board = Board {
            cur_state: [[0; 8]; 8],
            goal_state: [
                [0, 1, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 1, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0, 0, 0, 0],
            ],
            // goal_state: [[0,1,1,0,1,1,1,1],[1,0,0,1,0,0,0,0],[0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0]],
            moves: Vec::with_capacity(8),
        };

        #[cfg(debug_assertions)]
        {
            EightQueen::set_with_csv(&mut board, csv_init_data);
        }
        #[cfg(not(debug_assertions))]
        {
            EightQueen::fast_set_with_csv(&mut board, csv_init_data);
        }
        board
    }
    fn set_with_csv(&mut self, csv_init_data: &str) {
        EightQueen::fast_set_with_csv(self, csv_init_data);
    }
    #[inline(always)]
    fn fast_set_with_csv(&mut self, csv_init_data: &str) {
        let insert_data = |csv_data: &str, dest: &mut [[u8; 8]; 8]| {
            let csv_bytes = csv_data.as_bytes();
            let mut idx = 0;

            while idx < 8 * 3 - 1 {
                let file = csv_bytes[idx] - b'a';
                let rank = csv_bytes[idx + 1] - b'1'; // TODO: when N > 9
                dest[rank as usize][file as usize] = 1;
                idx += 3; // Skips a comma too.
            }
        };

        insert_data(csv_init_data, &mut self.cur_state);
    }
}

impl<const N: usize> Board<N> {
    #[must_use]
    /// The constructor for the Board struct.
    #[allow(dead_code)]
    pub fn new(csv_init_data: &str, csv_goal_data: &str) -> Board<N> {
        let mut board = Board {
            cur_state: [[0; N]; N],
            goal_state: [[0; N]; N],
            moves: Vec::with_capacity(N),
        };

        #[cfg(debug_assertions)]
        {
            board.set_with_csv(csv_init_data, csv_goal_data).unwrap();
        }
        #[cfg(not(debug_assertions))]
        {
            unsafe {
                board.fast_set_with_csv(csv_init_data, csv_goal_data);
            }
        }
        board
    }
    #[allow(dead_code)]
    pub fn set_with_csv(&mut self, csv_init_data: &str, csv_goal_data: &str) -> Result<(), ()> {
        // TODO: implement the safe version.
        unsafe {
            self.fast_set_with_csv(csv_init_data, csv_goal_data);
        }
        Ok(())
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn fast_set_with_csv(&mut self, csv_init_data: &str, csv_goal_data: &str) {
        let insert_data = |csv_data: &str, dest: &mut [[u8; N]; N]| {
            let csv_bytes = csv_data.as_bytes();
            let mut idx = 0;

            while idx < N * 3 - 1 {
                let file = csv_bytes[idx] - b'a';
                let rank = csv_bytes[idx + 1] - b'1'; // TODO: when N > 9
                dest[rank as usize][file as usize] = 1;
                idx += 3; // Skips a comma too.
            }
        };

        insert_data(csv_init_data, &mut self.cur_state);
        insert_data(csv_goal_data, &mut self.goal_state);
    }
    #[allow(dead_code)]
    pub fn set_with_fen(&mut self, fen_data: &str) -> Result<(), ()> {
        // TODO: Implement the safe version.
        unsafe {
            self.fast_set_with_fen(fen_data);
        }
        Ok(())
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn fast_set_with_fen(&mut self, fen_data: &str) {
        let mut raw_cur_state: *mut u8 = &mut self.cur_state[0][0];
        let fen_data = fen_data.as_bytes();

        let mut idx = 0;
        while fen_data[idx] != b' ' {
            if fen_data[idx] & 0x60 != 0 {
                *raw_cur_state = 1;
                raw_cur_state = raw_cur_state.add(1);
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
                raw_cur_state = raw_cur_state.add(n as usize);
            }
        }
    }
    /// Moves the selected chess piece to the given location, from the given chess coordinates notations.
    ///
    /// NOTE: This function does not check for the move validity, and will just move them regardless.
    ///
    /// # Examples:
    /// Basic usage:
    /// ```
    /// move_piece_with_coords("a2", "a4");
    /// ```
    #[allow(dead_code)]
    fn move_piece_with_coords(&mut self, src: &str, dest: &str) {
        let src = src.as_bytes();
        let dest = dest.as_bytes();

        let s = ((src[0] - b'a') as usize, (src[1] - b'1') as usize);
        let d = ((dest[0] - b'a') as usize, (dest[1] - b'1') as usize);

        if self.cur_state[s.1][s.0] == 1 {
            self.cur_state[s.1][s.0] = 0;
            self.cur_state[d.1][d.0] = 1;
        } else {
            println!("Move invalid");
        }
    }
    /// Returns an N-array of a row and column tuple of the queens position on the board.
    fn get_queens_pos(map: [[u8; N]; N]) -> [Coord; N] {
        let mut queens_pos = [Coord { row: 0, col: 0 }; N];
        let mut idx = 0;

        for (row_n, row) in map.iter().enumerate().rev() {
            for (col_n, val) in row.iter().enumerate() {
                if val == &1 {
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
        queens_pos.sort_unstable_by_key(|x| x.col);
        queens_pos
    }
    #[inline(always)]
    pub fn solve(&mut self) -> u16 {
        let mut stack = Vec::with_capacity(128);

        let queens = Self::get_queens_pos(self.cur_state);
        let mut solved = Self::get_queens_pos(self.goal_state);
        let mut defined_dest = [-1; N];

        for (i, x) in queens.iter().enumerate() {
            if self.goal_state[x.row as usize][x.col as usize] == 1 {
                solved[i] = Coord { row: -1, col: -1 };
                defined_dest[i] = i8::MAX;
            }
        }

        stack.push((queens, defined_dest, 0, 0));

        let mut lowest_moves = u16::MAX;
        while let Some((queens, defined_dest, solve_idx, moves)) = stack.pop() {
            if solve_idx == N {
                if moves < lowest_moves {
                    lowest_moves = moves;
                }
                continue;
            }
            let mut next_solve_idx = solve_idx + 1;
            while next_solve_idx < N && solved[next_solve_idx].row == -1 {
                next_solve_idx += 1;
            }

            for (i, x) in defined_dest.iter().enumerate() {
                if x == &-1 {
                    let mut defined_dest_new = defined_dest;
                    let mut queens_new = queens;
                    defined_dest_new[i] = solve_idx as i8;

                    let min = Self::min_moves_with_list(&queens, queens_new[i], solved[solve_idx]);
                    queens_new[i] = solved[solve_idx];
                    stack.push((queens, defined_dest_new, next_solve_idx, moves + min));
                }
            }
        }

        lowest_moves
    }
    /// XXX: $dest_square must not contain a Queen piece on that coordinates.
    fn min_moves_with_list(map_list: &[Coord; N], src_piece: Coord, dest_square: Coord) -> u16 {
        // The maximum steps allowed will always be a 3, if there is a path and, if the board is NxN with N queens.
        #[cfg(debug_assertions)]
        {
            for x in map_list {
                debug_assert!(*x != dest_square, "$dest_square must not contain a Queen!");
            }
        }

        // Define the $src_piece's and $dest_square's relative position on the board for easier comparison.
        // The board representation is not the same as a real chess board. The rows are mirrored, so rank 1 is 0th index and rank 8 is 7th index.
        // Example:
        //   -----     [
        // 2 | | |       [ 'Q', 'Q'],
        //   --+--       [ ' ', ' ']
        // 1 |Q|Q| is  ]
        //   -----
        //    a b        In memory
        let (src, dest) = (src_piece, dest_square);
        let (left, right) = if src.col > dest.col {
            (dest, src)
        } else {
            (src, dest)
        };
        let (bottom, top) = if src.row > dest.row {
            (dest, src)
        } else {
            (src, dest)
        };

        // These diagonals/slope are relative to the leftmost endpoint.
        let mut _bot_left_slope = Coord { row: 0, col: 0 };
        let mut _bot_right_slope = Coord {
            row: 0,
            col: N as i8,
        };
        let mut _top_left_slope = Coord {
            row: N as i8,
            col: 0,
        };
        let mut _top_right_slope = Coord {
            row: N as i8,
            col: N as i8,
        };

        for x in map_list {}

        let is_inbetween = |low, mid, high| low < mid && mid < high;
        let is_inbetween_unordered =
            |end, mid, end2| is_inbetween(end, mid, end2) || is_inbetween(end2, mid, end);
        // TODO: might try to see if storing the pieces that have the same row/column in a vec is more performant.

        let mut min_path = u16::MAX;

        // Trying min=1.
        if src.row == dest.row {
            for x in map_list {
                if x.row == src.row && !is_inbetween(bottom.col, x.col, top.col) {
                    // $x is in between the path?
                    min_path = 1;
                    break;
                }
            }
        } else if src.col == dest.col {
            for x in map_list {
                if x.col == src.col && !is_inbetween(left.row, x.row, right.row) {
                    // $x is in between the path?
                    min_path = 1;
                    break;
                }
            }
        } else if top.row - bottom.row == right.col - left.col {
            // TODO: check path is obstructed.
            min_path = 1;
        }

        if min_path != u16::MAX {
            return min_path;
        }
        return 2;
        /*

                // Trying 2.

                // for x in map_list {
                //     if x.col > top_right_slope.col
                //         && x.row > top_right_slope.row
                //         && x.row - top_right_slope.row == x.col - top_right_slope.col
                //     {
                //         top_right_slope = *x;
                //     } else if x.col > bot_right_slope.col
                //         && bot_right_slope.row > x.row
                //         && x.col - bot_right_slope.col == bot_right_slope.row - x.row
                //     {
                //         bot_right_slope = *x;
                //     }
                // }

                // TODO: Do dope slope stuff
                if bot_right_slope.col >= right.col {
                    let mut valid = true;
                    for x in map_list {
                        if x.col == right.col && is_inbetween_unordered(bot_right_slope.row, x.row, right.row) {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        min_path = 2;
                    }
                } else if top_right_slope.col >= right.col {
                    let mut valid = true;
                    for x in map_list {
                        if x.col == right.col && is_inbetween_unordered(top_right_slope.row, x.row, right.row) {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        min_path = 2;
                    }
                }
        */
        min_path
    }
    pub fn to_string(&self) -> String {
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
                layout[cal!(y, x)] = b'*';
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

        for (row_n, row) in self.cur_state[..].iter().rev().enumerate() {
            for (col_n, val) in row.iter().enumerate() {
                if val == &1 {
                    layout[cal!(row_n, col_n)] = b'Q';
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
