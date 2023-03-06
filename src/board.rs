#[allow(dead_code)]
pub struct Board<const N: usize> {
    cur_state: [[u8; N]; N],
    goal_state: [[u8; N]; N],
    moves: Vec<String>,
}

#[derive(Copy, Clone)]
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
            board.fast_set_with_csv(csv_init_data, csv_goal_data);
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
        // Iterative deepening.
        for i in 1..N as u16 * 2 {
            let t = std::time::Instant::now();
            if self.solve_inner(i) != i {
                println!(
                    "{} end, used {}ms or {}us",
                    i,
                    t.elapsed().as_millis(),
                    t.elapsed().as_micros()
                );
                return i;
            }
            println!(
                "{} end, used {}ms or {}us",
                i,
                t.elapsed().as_millis(),
                t.elapsed().as_micros()
            );
        }
        u16::MAX
        // self.solve_inner(N as u16*2)
    }
    #[inline(always)]
    pub fn solve_inner(&mut self, cutoff: u16) -> u16 {
        let mut stack = Vec::with_capacity(64);

        let map = Self::get_queens_pos(&self.cur_map);
    }
    fn min_step_with_list(map_list: &[Coord; N], src: Coord, dest: Coord) -> u16 {
        // The maximum steps allowed will always be a 3. If board is NxN with N queens.
        // Define the $src and $dest relative position on the board for easier comparison.
        // The board representation is not the same as a real chess board. The rows are mirrored, so rank 1 is 0th index and rank 8 is 7th index.
        // Example:
        //   -----    [
        // 2 | | |      [ 'Q', 'Q'],
        //   --+--      [ ' ', ' ']
        // 1 |Q|Q| is ]
        //   -----
        //    a b        In memory
        let node_left = if src.col < dest.col { src } else { dest };
        let node_right = if src.col > dest.col { src } else { dest };
        let node_top = if src.row > dest.row { src } else { dest };
        let node_bottom = if src.row < dest.row { src } else { dest };

        // Trying min=1.
        let mut path_1_exist = true;
        if src.row == dest.row {
            for x in map_list {
                if x.row == src.row && node_top.col > x.col && x.col > node_bottom.col {
                    // $x is in between the path?
                    path_exist = false;
                    break;
                }
            }
        } else if src.col == dest.col {
            for x in map_list {
                if x.col == src.col && node_left.row > x.row && x.row > node_rigth.row {
                    // $x is in between the path?
                    path_exist = false;
                    break;
                }
            }
        }
    }
}

impl<const N: usize> std::fmt::Display for Board<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
