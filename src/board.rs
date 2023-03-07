#[allow(dead_code)]
pub struct Board<const N: usize> {
    cur_state: [[u8; N]; N],
    goal_state: [[u8; N]; N],
    moves: Vec<String>,
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Coord {
    row: i8,
    col: i8,
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: deal with board size > 'z'.
        write!(
            f,
            "{}{}",
            (self.col as u8 + b'a') as char,
            (self.row as u8 + b'1') as char
        )
    }
}

// TODO: Maybe generate all moveable moves so that an additional Coord type is not required.
// TOOO: List out the 3-moves moves.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[allow(dead_code)]
pub enum Moves {
    Horizontal(Coord, Coord),
    Vertical(Coord, Coord),
    Diagonal(Coord, Coord),
    ThreeMoves(Coord, Coord),
    NoPossibleMoves,
}

#[allow(dead_code)]
impl Moves {
    pub fn value(&self) -> u16 {
        use Moves::*;
        match self {
            Horizontal(_, _) | Vertical(_, _) | Diagonal(_, _) => 1,
            ThreeMoves(_, _) => 3,
            NoPossibleMoves => u16::MAX,
        }
    }
    pub fn get_values(&self) -> Option<(Coord, Coord)> {
        use Moves::*;
        match self {
            Horizontal(src, dest)
            | Vertical(src, dest)
            | Diagonal(src, dest)
            | ThreeMoves(src, dest) => Some((*src, *dest)),
            NoPossibleMoves => None,
        }
    }
    pub fn get_src(&self) -> Option<Coord> {
        Some(self.get_values()?.0)
    }
    pub fn get_dest(&self) -> Option<Coord> {
        Some(self.get_values()?.1)
    }
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
    pub fn solve(&mut self) -> Vec<Moves> {
        let mut stack = Vec::with_capacity(128);

        let queens = Self::get_queens_pos(self.cur_state);
        let mut solved = Self::get_queens_pos(self.goal_state);
        let mut defined_dest = [-1; N];

        for (i, x) in queens.iter().enumerate() {
            for y in solved.iter_mut() {
                if x == y {
                    *y = Coord { row: -1, col: -1 };
                    defined_dest[i] = i8::MAX;
                }
            }
        }

        let mut solve_idx = 0;
        while solve_idx < N && solved[solve_idx].row == -1 {
            solve_idx += 1;
        }
        stack.push((queens, defined_dest, solve_idx, Vec::with_capacity(N)));

        let mut lowest_moves = u16::MAX;
        let mut lowest_moves_list = Vec::new();

        while let Some((queens, defined_dest, solve_idx, moves)) = stack.pop() {
            if solve_idx == N {
                let cur_moves_value = moves.iter().fold(0, |acc, x: &Moves| acc + x.value());
                if cur_moves_value < lowest_moves {
                    lowest_moves = cur_moves_value;
                    lowest_moves_list = moves;
                }
                continue;
            }
            let mut next_solve_idx = solve_idx + 1;
            while next_solve_idx < N && solved[next_solve_idx].row == -1 {
                next_solve_idx += 1;
            }

            for (i, x) in defined_dest.iter().enumerate() {
                if *x == -1 {
                    let mut defined_dest_new = defined_dest;
                    let mut queens_new = queens;
                    let mut moves_new = moves.clone();

                    let possible = Self::min_moves_with_list(
                        &queens,
                        queens[i],
                        solved[solve_idx],
                        &mut moves_new,
                    );
                    if possible {
                        defined_dest_new[i] = solve_idx as i8;
                        queens_new[i] = solved[solve_idx];
                    }
                    stack.push((queens_new, defined_dest_new, next_solve_idx, moves_new));
                }
            }
        }

        lowest_moves_list
    }
    /// XXX: $dest_square must not contain a Queen piece on that coordinates.
    fn min_moves_with_list(
        map_list: &[Coord; N],
        src_piece: Coord,
        dest_square: Coord,
        moves: &mut Vec<Moves>,
    ) -> bool {
        // The maximum steps allowed will always be a 3, if there is a path and, if the board is NxN with N queens.
        #[cfg(debug_assertions)]
        {
            for x in map_list {
                debug_assert!(*x != dest_square, "$dest_square must not contain a Queen!");
            }
            debug_assert!(
                src_piece != dest_square,
                "$src_piece and $dest_square should not be the same!"
            );
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
        let mut bot_left_slope = left;
        let mut bot_right_slope = left;
        let mut top_left_slope = left;
        let mut top_right_slope = left;

        {
            let top = N as i8 - left.row - 1;
            let bottom = left.row;
            let right = N as i8 - left.col - 1;
            let left = left.col;

            bot_left_slope.row -= bottom.min(left) + 1;
            bot_left_slope.col -= bottom.min(left) + 1;

            bot_right_slope.row -= bottom.min(right) + 1;
            bot_right_slope.col += bottom.min(right) + 1;

            top_left_slope.row += top.min(left) + 1;
            top_left_slope.col -= top.min(left) + 1;

            top_right_slope.row += top.min(right) + 1;
            top_right_slope.col += top.min(right) + 1;
        }

        for x in map_list.iter().chain([right].iter()) {
            if x.row > left.row {
                if x.col > left.col {
                    if x.row - left.row == x.col - left.col && x.row < top_right_slope.row {
                        top_right_slope = *x;
                    }
                } else {
                    if x.row - left.row == left.col - x.col && x.row < top_left_slope.row {
                        top_left_slope = *x;
                    }
                }
            } else if x.col > left.col {
                if left.row - x.row == x.col - left.col && x.row > bot_right_slope.row {
                    bot_right_slope = *x;
                }
            } else {
                if left.row - x.row == left.col - x.col && x.row > bot_left_slope.row {
                    bot_left_slope = *x;
                }
            }
        }

        let is_inbetween = |low, mid, high| low < mid && mid < high;
        let is_inbetween_unordered =
            |end, mid, end2| is_inbetween(end, mid, end2) || is_inbetween(end2, mid, end);

        // TODO: might try to see if storing the pieces that have the same row/column in a vec is more performant.

        // Trying min=1.
        if src.row == dest.row {
            let mut valid = true;
            for x in map_list {
                if x.row == src.row && is_inbetween(left.col, x.col, right.col) {
                    valid = false;
                    break;
                }
            }
            if valid {
                moves.push(Moves::Horizontal(src, dest));
                return true;
            }
        } else if src.col == dest.col {
            let mut valid = true;
            for x in map_list {
                if x.col == src.col && is_inbetween(bottom.row, x.row, top.row) {
                    valid = false;
                    break;
                }
            }
            if valid {
                moves.push(Moves::Vertical(src, dest));
                return true;
            }
        } else if (left == dest
            && (bot_left_slope == src
                || bot_right_slope == src
                || top_left_slope == src
                || top_right_slope == src))
            || (left == src
                && (bot_left_slope == dest
                    || bot_right_slope == dest
                    || top_left_slope == dest
                    || top_right_slope == dest))
        {
            moves.push(Moves::Diagonal(src, dest));
            return true;
        }

        let slope_get_row = |slope: Coord, col| {
            let orig = left;
            if col > orig.col {
                if slope.row > orig.row {
                    // Top right
                    orig.row + (col - orig.col)
                } else {
                    // Bottom right
                    orig.row - (col - orig.col)
                }
            } else if slope.row > orig.row {
                // Top left
                orig.row + (orig.col - col)
            } else {
                // Bottom left
                orig.row - (orig.col - col)
            }
        };

        let slope_get_col = |slope: Coord, row| {
            let orig = left;
            if row > orig.row {
                if slope.col > orig.col {
                    // Top right
                    orig.col + (row - orig.row)
                } else {
                    // Top left
                    orig.col - (row - orig.row)
                }
            } else if slope.col > orig.col {
                // Bottom right
                orig.col + (orig.row - row)
            } else {
                // Bottom left
                orig.col - (orig.col - row)
            }
        };

        let mut capture = |slope| {
            (
                slope_get_row,
                slope_get_col,
                is_inbetween_unordered,
                map_list,
                slope,
                right,
                src,
                dest,
                moves as *mut Vec<_>,
            )
        };
        macro_rules! enter_slope {
            (vertically $capture: expr) => {{
                let (fn1, _, fn3, a, b, c, d, e, f) = $capture;
                let intersection = fn1(b, c.col);
                enter_slope!(fn3, a, c, d, e, intersection, f, row, col, Vertical)
            }};
            (horizontally $capture: expr) => {{
                let (_, fn2, fn3, a, b, c, d, e, f) = $capture;
                let intersection = fn2(b, c.row);
                enter_slope!(fn3, a, c, d, e, intersection, f, col, row, Horizontal)
            }};
            ($is_inbetween_unordered: expr, $map_list: expr, $right: expr, $src: expr, $dest: expr, $intersection: expr, $moves: expr, $main_dir: tt, $opp_dir: tt, $direction: tt) => {{
                let mut valid = true;
                for x in $map_list {
                    if x.$opp_dir == $right.$opp_dir
                        && $is_inbetween_unordered($intersection, x.$main_dir, $right.$main_dir)
                    {
                        valid = false;
                        break;
                    }
                }

                use Moves::*;
                if valid {
                    let intersection = Coord {
                        $main_dir: $intersection,
                        $opp_dir: $right.$opp_dir,
                    };

                    let moves = unsafe { $moves.as_mut().unwrap() };
                    if $right == $src {
                        moves.push($direction($src, intersection));
                        moves.push(Diagonal(intersection, $dest));
                    } else {
                        moves.push(Diagonal($src, intersection));
                        moves.push($direction(intersection, $dest));
                    }
                }
                valid
            }};
        }

        // Trying 2.
        if bot_right_slope.col > right.col {
            if enter_slope!(vertically capture(bot_right_slope)) {
                return true;
            }
        // }
        } else if top_right_slope.col >= right.col {
            // if top_right_slope.col > right.col {
            if enter_slope!(vertically capture(top_right_slope)) {
                return true;
            }
        // }
        } else if top_right_slope.row >= right.row {
            // if top_right_slope.row > right.row {
            if enter_slope!(horizontally capture(top_right_slope)) {
                return true;
            }
        // }
        } else if top_left_slope.row >= right.row {
            // if top_left_slope.row > right.row {
            if enter_slope!(horizontally capture(top_left_slope)) {
                return true;
            }
        // }
        } else if bot_left_slope.row >= right.row {
            // if bot_left_slope.row > right.row {
            if enter_slope!(horizontally capture(bot_left_slope)) {
                return true;
            }
        // }
        } else if bot_right_slope.row >= right.row {
            // if bot_right_slope.row > right.row {
            if enter_slope!(horizontally capture(bot_right_slope)) {
                return true;
            }
        }
        // TODO: Diagonal to Diagonal move

        // TODO: check if path exist.
        let path_exist = true;
        if path_exist {
            moves.push(Moves::ThreeMoves(src, dest));
            true
        } else {
            false
        }
    }
    pub fn print_moves(&mut self, moves: &Vec<Moves>) {
        let mut map = self.cur_state;
        for (i, x) in moves.iter().enumerate() {
            if let Some((src, dest)) = x.get_values() {
                map[src.row as usize][src.col as usize] = 0;
                map[dest.row as usize][dest.col as usize] = 1;
            }

            println!("{}\n", Self::to_string_inner(&map));
            println!("Move {}: {x:?}\n\n", i + 1);
        }
    }
    pub fn to_string(&self) -> String {
        Self::to_string_inner(&self.cur_state)
    }
    fn to_string_inner(map: &[[u8; N]; N]) -> String {
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

        for (row_n, row) in map[..].iter().rev().enumerate() {
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
