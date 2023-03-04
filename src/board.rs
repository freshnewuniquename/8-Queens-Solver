#[allow(dead_code)]
pub struct Board<const N: usize> {
    cur_state: [[u8; N]; N],
    goal_state: [[u8; N]; N],
    moves: Vec<String>,
}

#[derive(Copy, Clone)]
struct Coord {
    row: i8,
    col: i8
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
            goal_state: [[0,1,0,0,0,0,0,0],[0,0,0,0,1,0,0,0],[0,0,0,0,0,0,1,0],[0,0,0,1,0,0,0,0],[1,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,1],[0,0,0,0,0,1,0,0],[0,0,1,0,0,0,0,0]],
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

            while idx < 8*3-1 {
                let file = csv_bytes[idx]-b'a';
                let rank = csv_bytes[idx+1]-b'1'; // TODO: when N > 9
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
        unsafe { self.fast_set_with_csv(csv_init_data, csv_goal_data); }
        Ok(())
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub unsafe fn fast_set_with_csv(&mut self, csv_init_data: &str, csv_goal_data: &str) {
        let insert_data = |csv_data: &str, dest: &mut [[u8; N]; N]| {
            let csv_bytes = csv_data.as_bytes();
            let mut idx = 0;

            while idx < N*3-1 {
                let file = csv_bytes[idx]-b'a';
                let rank = csv_bytes[idx+1]-b'1'; // TODO: when N > 9
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
        unsafe { self.fast_set_with_fen(fen_data); }
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
    #[allow(dead_code)]
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
    fn get_queens_pos(map: [[u8; N]; N]) -> [Coord; N] {
        let mut queens_pos = [Coord { row: 0, col: 0 }; N];
        let mut idx = 0;

        for (row_n, row) in map.iter().enumerate().rev() {
            for (col_n, val) in row.iter().enumerate() {
                if val == &1 {
                    if idx == N {
                        break;
                    }
                    queens_pos[idx] = Coord { row: row_n as i8, col: col_n as i8 };
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
        for i in 1..N as u16*2 {
            let t = std::time::Instant::now();
            if self.solve_inner(i) != i {
                println!("{} end, used {}ms or {}us", i, t.elapsed().as_millis(), t.elapsed().as_micros());
                return i;
            }
            println!("{} end, used {}ms or {}us", i, t.elapsed().as_millis(), t.elapsed().as_micros());
        }
        u16::MAX
        // self.solve_inner(N as u16*2)
    }
    #[inline(always)]
    pub fn solve_inner(&mut self, cutoff: u16) -> u16 {
        let mut stack = Vec::with_capacity(64);
        // let mut stack = std::collections::VecDeque::with_capacity(100_000_000);
        let map = Board::get_queens_pos(self.cur_state);
        if self.validate_list(&map) {
            return 0;
        }

        let mut lowest_solve = cutoff;
        let mut lowest_solve_map = [Coord { row: i8::MAX, col: i8::MAX}; N];

        let mut visited = [(0, 0, 0, 0); N]; // 0: Top, 1: Bottom, 2: Right, 3: Left
        for (visited_x, queens_x) in visited.iter_mut().zip(map) {
            visited_x.0 = queens_x.row;
            visited_x.1 = queens_x.row;
            visited_x.2 = queens_x.col;
            visited_x.3 = queens_x.col;
        }

        stack.push((N, map, 0, visited));
        // stack.push_back((N, map, 0, visited));

        /*
            Benchmark of the current revision:
            ==================================
            (Using replit.com's server CPU)
            Iterative deepening Depth-first search:
              -----------------
            8 | |*| |*| |*| |*|
              --+-+-+-+-+-+-+--
            7 |*| |*| |*| |*| |
              --+-+-+-+-+-+-+--
            6 | |*| |*| |*| |*|
              --+-+-+-+-+-+-+--
            5 |*| |*| |*| |*| |
              --+-+-+-+-+-+-+--
            4 | |*| |*| |*| |*|
              --+-+-+-+-+-+-+--
            3 |*| |*| |*| |*| |
              --+-+-+-+-+-+-+--
            2 | |*| |*| |*| |*|
              --+-+-+-+-+-+-+--
            1 |Q|Q|Q|Q|Q|Q|Q|Q|
              -----------------
               a b c d e f g h  
            1 end, used 0ms or 6us
            2 end, used 0ms or 26us
            3 end, used 0ms or 591us
            4 end, used 8ms or 8380us
            5 end, used 227ms or 227277us
            6 end, used 2477ms or 2477150us
            7 end, used 13803ms or 13803725us
            8 end, used 103219ms or 103219921us
            moves: 8
              -----------------
            8 | |*|Q|*| |*| |*|
              --+-+-+-+-+-+-+--
            7 |*| |*| |*|Q|*| |
              --+-+-+-+-+-+-+--
            6 | |*| |*| |*| |Q|
              --+-+-+-+-+-+-+--
            5 |Q| |*| |*| |*| |
              --+-+-+-+-+-+-+--
            4 | |*| |Q| |*| |*|
              --+-+-+-+-+-+-+--
            3 |*| |*| |*| |Q| |
              --+-+-+-+-+-+-+--
            2 | |*| |*|Q|*| |*|
              --+-+-+-+-+-+-+--
            1 |*|Q|*| |*| |*| |
              -----------------
               a b c d e f g h  
            
            real    1m59.741s
            user    0m47.392s
            sys 0m1.040s
        */

        'main: while let Some((prev, map, n, mut visited)) = stack.pop() {
            // 'main: while let Some((prev, map, n, mut visited)) = stack.pop_front() {
            if n >= lowest_solve {
                // Pruning.
                continue;
            }

            for queen_i in 0..N {
                if queen_i != prev  {
                    let cur_row = map[queen_i].row;
                    let cur_col = map[queen_i].col;

                    // Define these first, since $visited's values may be changed later.
                    let mut row_up_i = visited[queen_i].0 + 1;
                    let mut row_down_i = visited[queen_i].1 - 1;
                    let mut col_right_i = visited[queen_i].2 + 1;
                    let mut col_left_i = visited[queen_i].3 - 1;

                    // Set flags first, before it gets overwritten.
                    let should_go_up = visited[queen_i].0 != N as i8;
                    let should_go_down = visited[queen_i].1 != -1;
                    let should_go_right = visited[queen_i].2 != N as i8;
                    let should_go_left = visited[queen_i].3 != -1;

                    // FIXME: This is no longer correct when the column or row changes.
                    // Set all of the current available queen move's boundary to memory, so that in future moves, the queen will not be moved to the moveable location again.
                    // For example: current queen occupies b2, and can move unobstructed, except b3 is obstructed.
                    // All of the unobstructed moves are recorded.
                    // On the next move to move the same queen, it will check if there are any previously unobstructed squares she can move to on the current move.
                    // Lets say the b3 queen has moved, on the current move, the queen will only move upwards until she is obstructed again. Then she records which coordinates she is able to go to again.
                    let mut traversable_up = N as i8;
                    let mut traversable_down = -1;
                    for queen_x in map {
                        if queen_x.col == cur_col {
                            if queen_x.row > cur_row && queen_x.row < traversable_up {
                                traversable_up = queen_x.row;
                            } else if queen_x.row < cur_row && queen_x.row > traversable_down {
                                traversable_down = queen_x.row;
                            }
                        }
                    }
                    visited[queen_i].0 = traversable_up;
                    visited[queen_i].1 = traversable_down;

                    let mut traversable_right = N as i8;
                    let mut traversable_left = -1;
                    for queen_x in map {
                        if queen_x.row == cur_row {
                            if queen_x.col > cur_col && queen_x.col < traversable_right {
                                traversable_right = queen_x.col;
                            } else if queen_x.col < cur_col && queen_x.col > traversable_left {
                                traversable_left = queen_x.col;
                            }
                        }
                    }
                    visited[queen_i].2 = traversable_right;
                    visited[queen_i].3 = traversable_left;


                    // Search the map, one step at a time, upward.
                    if should_go_up {
                        while row_up_i < traversable_up {
                            let mut new_map = map.clone();
                            let new_visited = visited.clone();
                            new_map[queen_i].row = row_up_i;
                            row_up_i += 1;

                            if self.validate_list(&new_map) {
                                lowest_solve = n+1;
                                lowest_solve_map = new_map;
                                break 'main;
                            }
                            stack.push((queen_i, new_map, n+1, new_visited));
                            // stack.push_back((queen_i, new_map, n+1, new_visited));

                            // println!("{} moves:{}", Self::to_string_list(&new_map), n+1);
                            // std::thread::sleep(std::time::Duration::new(0, 200_000_000));
                        }
                    }

                    // Search the map, downwards.
                    if should_go_down {
                        while row_down_i > traversable_down {
                            let mut new_map = map.clone();
                            let new_visited = visited.clone();
                            new_map[queen_i].row = row_down_i;
                            row_down_i -= 1;

                            if self.validate_list(&new_map) {
                                lowest_solve = n+1;
                                lowest_solve_map = new_map;
                                break 'main;
                            }
                            stack.push((queen_i, new_map, n+1, new_visited));
                            // stack.push_back((queen_i, new_map, n+1, new_visited));

                            // println!("{} moves:{}", Self::to_string_list(&new_map), n+1);
                            // std::thread::sleep(std::time::Duration::new(0, 200_000_000));
                        }
                    }

                    // Search the map, right.
                    if should_go_right {
                        while col_right_i < traversable_right {
                            let mut new_map = map.clone();
                            let new_visited = visited.clone();
                            new_map[queen_i].col = col_right_i;
                            col_right_i += 1;

                            if self.validate_list(&new_map) {
                                lowest_solve = n+1;
                                lowest_solve_map = new_map;
                                break 'main;
                            }
                            stack.push((queen_i, new_map, n+1, new_visited));
                            // stack.push_back((queen_i, new_map, n+1, new_visited));

                            // println!("{} moves:{}", Self::to_string_list(&new_map), n+1);
                            // std::thread::sleep(std::time::Duration::new(0, 200_000_000));
                        }
                    }

                    // Search the map, left.
                    if should_go_left {
                        while col_left_i > traversable_left {
                            let mut new_map = map.clone();
                            let new_visited = visited.clone();
                            new_map[queen_i].col = col_left_i;
                            col_left_i -= 1;

                            if self.validate_list(&new_map) {
                                lowest_solve = n+1;
                                lowest_solve_map = new_map;
                                break 'main;
                            }
                            stack.push((queen_i, new_map, n+1, new_visited));
                            // stack.push_back((queen_i, new_map, n+1, new_visited));

                            // println!("{} moves:{}", Self::to_string_list(&new_map), n+1);
                            // std::thread::sleep(std::time::Duration::new(0, 200_000_000));
                        }
                    }
                }
            }
        }

        if lowest_solve != cutoff {
            for row in &mut self.cur_state {
                for val in row {
                    if val == &1 {
                        *val = 0;
                    }
                }
            }
            for x in lowest_solve_map {
                self.cur_state[x.row as usize][x.col as usize] = 1;
            }
        }
        lowest_solve
    }
    #[inline(always)]
    fn validate_list(&self, queens_pos: &[Coord; N]) -> bool {
        for x in queens_pos {
            unsafe {
                if self.goal_state.get_unchecked(x.row as usize).get_unchecked(x.col as usize) == &0 {
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
    pub fn to_string(&self) -> String {
        Self::to_string_inner(&self.cur_state)
    }
    #[allow(dead_code)]
    fn to_string_list(list: &[Coord; N]) -> String {
        let mut map = [[0; N]; N];
        for x in list {
            map[x.row as usize][x.col as usize] = 1;
        }
        Self::to_string_inner(&map)
    }
    fn to_string_inner(cur_state: &[[u8; N]; N]) -> String { // TODO: const generate the $layout.
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
            for x in ((y+1)%2..N).step_by(2) {
                layout[cal!(y, x)] = b'*';
            }
        }

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

        for (row_n, row) in cur_state[..].iter().rev().enumerate() {
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