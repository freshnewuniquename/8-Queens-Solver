pub struct Board<const N: usize> {
    cur_state: [[u8; N]; N],
    goal_state: [[u8; N]; N],
    moves: Vec<String>,
}

pub trait EightQueen {
    fn new(csv_set_data: &str) -> Board<8>;
    fn set_with_csv(&mut self, csv_set_data: &str);
    fn fast_set_with_csv(&mut self, csv_set_data: &str);
}

impl EightQueen for Board<8> {
    fn new(csv_set_data: &str) -> Board<8> {
        let mut board = Board {
            cur_state: [[0; 8]; 8],
            goal_state: [[0,1,0,0,0,0,0,0],[0,0,0,0,1,0,0,0],[0,0,0,0,0,0,1,0],[0,0,0,1,0,0,0,0],[1,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,1],[0,0,0,0,0,1,0,0],[0,0,1,0,0,0,0,0]],
            moves: Vec::with_capacity(8),
        };

        #[cfg(debug_assertions)]
        {
            EightQueen::set_with_csv(&mut board, csv_set_data);
        }
        #[cfg(not(debug_assertions))]
        {
            EightQueen::fast_set_with_csv(&mut board, csv_set_data);
        }
        board
    }
    fn set_with_csv(&mut self, csv_set_data: &str) {
        EightQueen::fast_set_with_csv(self, csv_set_data);
    }
    #[inline(always)]
    fn fast_set_with_csv(&mut self, csv_set_data: &str) {
        let insert_data = |csv_data: &str, dest: &mut [[u8; 8]; 8]| {
            let csv_bytes = csv_data.as_bytes();
            let mut idx = 0;
            
            while idx < 8*3-1 {
                let file = csv_bytes[idx]-b'a';
                let rank = csv_bytes[idx+1]-b'1'; // TODO: when N > 9
                dest[rank as usize][file as usize] = 1;
                idx += 3; // Skips a comma too.
            }
        };
        
        insert_data(csv_set_data, &mut self.cur_state);
    }
}

impl<const N: usize> Board<N> {
    #[must_use]
    /// The constructor for the Board struct.
    pub fn new(csv_set_data: &str, csv_goal_data: &str) -> Board<N> {
        let mut board = Board {
            cur_state: [[0; N]; N],
            goal_state: [[0; N]; N],
            moves: Vec::with_capacity(N)
        };
        
        #[cfg(debug_assertions)]
        {
            board.set_with_csv(csv_set_data, csv_goal_data);
        }
        #[cfg(not(debug_assertions))]
        {
            board.fast_set_with_csv(csv_set_data, csv_goal_data);
        }
        board
    }
    pub fn set_with_csv(&mut self, csv_set_data: &str, csv_goal_data: &str) -> Result<(), ()> {
        // TODO: implement the safe version.
        self.fast_set_with_csv(csv_set_data, csv_goal_data);
        Ok(())
    }
    #[inline(always)]
    pub fn fast_set_with_csv(&mut self, csv_set_data: &str, csv_goal_data: &str) {
        let insert_data = |csv_data: &str, dest: &mut [[u8; N]; N]| {
            let csv_bytes = csv_data.as_bytes();
            let mut idx = 0;
            
            while idx < 8*3-1 {
                let file = csv_bytes[idx]-b'a';
                let rank = csv_bytes[idx+1]-b'1'; // TODO: when N > 9
                dest[rank as usize][file as usize] = 1;
                idx += 3; // Skips a comma too.
            }
        };
        
        insert_data(csv_set_data, &mut self.cur_state);
        insert_data(csv_goal_data, &mut self.cur_state);
    }
    pub fn set_with_fen(&mut self, fen_data: &str) -> Result<(), ()> {
        todo!()
    }
    #[inline(always)]
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
                let mut n = fen_data[idx]-b'0';
                // XOR 0b0011_0000, to filter out non-digits.
                // 0 - 0b0011_0000
                // 9 - 0b0011_1001
                //   - 0b0010_0000
                // Q - 0b0101_0001
                while fen_data[idx+1] ^ 0x30 > 0x10 {
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
    fn move_piece_with_coords(&mut self, src: &str, dest: &str) {
        let src = src.as_bytes();
        let dest = dest.as_bytes();

        let s = ((src[0]-b'a') as usize, (src[1]-b'1') as usize);
        let d = ((dest[0]-b'a') as usize, (dest[1]-b'1') as usize);

        if self.cur_state[s.1][s.0] == 1 {
            self.cur_state[s.1][s.0] = 0;
            self.cur_state[d.1][d.0] = 1;
        } else {
            println!("Move invalid");
        }
    }
    /// Returns an N-array of a row and column tuple of the queens position on the board. 
    fn get_queens_pos(map: [[u8; N]; N]) -> [(u8, u8); N] {
        let mut queens_pos = [(0, 0); N];
        let mut idx = 0;
        
        for (row_n, row) in map.iter().enumerate().rev() {
            for (col_n, val) in row.iter().enumerate() {
                if val == &1 {
                    if idx == N {
                        break;
                    }
                    queens_pos[idx] = (row_n as u8, col_n as u8);
                    idx += 1;
                }
            }
        }
        queens_pos.sort_unstable_by_key(|x| x.1);
        queens_pos
    }
    #[inline(always)]
    pub fn solve(&mut self) -> u16 {
        let mut stack = Vec::with_capacity(64);

        let map = Board::get_queens_pos(self.cur_state);
        if self.validate_list(&map) {
            return 0;
        }
        
        let mut lowest_solve = u16::MAX;
        let mut lowest_solve_map = [(u8::MAX, u8::MAX); N];
        
        'main: while let Some((idx, map, n)) = stack.pop() {
            if n >= lowest_solve {
                // Pruning.
                continue;
            }

            for queen_i in idx..N {
                if queen_i != idx {
                    let mut row_i = 0;
                    
                    while row_i < N as u8 {
                        let mut new_map = map.clone();
                        new_map[queen_i].0 = row_i;
                        row_i += 1;

                        if self.validate_list(&new_map) {
                            lowest_solve = n+1;
                            lowest_solve_map = new_map;
                            continue 'main;
                        }
                        stack.push((queen_i, new_map, n+1));
                    }
                }
            }
        }
        if lowest_solve != u16::MAX {
            for row in &mut self.cur_state {
                for val in row {
                    if val == &1 {
                        *val = 0;
                    }
                }
            }
            for x in lowest_solve_map {
                self.cur_state[x.0 as usize][x.1 as usize] = 1;
            }
        }
        lowest_solve
    }
    #[inline(always)]
    fn validate_list(&self, queens_pos: &[(u8, u8); N]) -> bool {
        for x in queens_pos {
            unsafe {
                if self.goal_state.get_unchecked(x.0 as usize).get_unchecked(x.1 as usize) == &0 {
                    return false;
                }
            }
        }
        true
    }
    #[allow(dead_code)]
    pub fn validate_game(&self) -> bool {
        self.cur_state == self.goal_state
    }
    pub fn to_string(&self) -> String { // TODO: const generate the $layout.
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

        let mut i = 2;
        let mut cur_row = 0;
        while i < cal!(whole_len_no_file_indicators) {
            if cal!(is_newline i, cur_row) {
                if cur_row&1 == 0 {
                    layout[i] = b'-';
                    if cur_row != N*2 {
                        layout[i+2] = b'0' + N as u8 - cur_row as u8/2;
                    }
                } else {
                    layout[i] = b'|';
                }
                
                layout[i+1] = b'\n';
                cur_row += 1;
                i += 2;
            } else if cur_row&1 == 0 {
                if cal!(is_intersection i, cur_row) {
                    layout[i] = b'+';
                } else {
                    layout[i] = b'-';
                }

                layout[i+1] = b'-';
            } else {
                layout[i] = b'|';
            }
            
            i += 2;
        }

        i += 1;
        let mut char = b'a';
        while i < cal!(whole_len_no_file_indicators)+cal!(row_len_only_board)+1 {
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