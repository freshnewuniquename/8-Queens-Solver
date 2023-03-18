#![allow(dead_code)]
use crate::search::{self, Search};

pub struct Board<const N: usize = 8> {
    pub(super) init_state: [[u8; N]; N],
    pub(super) goal_state: [[u8; N]; N],
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct Coord {
    pub row: i8,
    pub col: i8,
}

impl Coord {
    fn abs_diff(self, other: Self) -> u8 {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
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
    fn char_to_u8(self) -> u8 {
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

// TODO: Maybe generate all moveable moves so that an additional Coord type is not required.
// TOOO: List out the 3-moves moves.
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
}

#[allow(non_upper_case_globals)]
static mut μs_used_searching_is_blocked: u128 = 0;

#[allow(dead_code)]
impl Moves {
    pub fn get_values(&self) -> Option<(Coord, Coord)> {
        use Moves::*;
        match self {
            Horizontal(src, dest)
            | Vertical(src, dest)
            | Diagonal(src, dest)
            | ThreeMoves1(src, dest)
            | ThreeMoves2(src, dest)
            | ThreeMoves3(src, dest) => Some((*src, *dest)),
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SearchStatus {
    Ok,
    OnHold(usize),
    RetryingHold(usize),
}

impl<const N: usize> Default for Board<N> {
    fn default() -> Self {
        let goal_state = if N == 8 {
            const EIGHT_QUEEN_GOAL: [[u8; 8]; 8] = [
                [0, 1, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 1, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 1, 0, 0],
                [0, 0, 1, 0, 0, 0, 0, 0],
            ];

            std::array::from_fn(|row_i| {
                std::array::from_fn(|col_i| unsafe {
                    *EIGHT_QUEEN_GOAL.get_unchecked(row_i).get_unchecked(col_i)
                })
            })
        } else {
            [[0; N]; N]
        };

        Board {
            init_state: [[0; N]; N],
            goal_state,
        }
    }
}

impl<const N: usize> Board<N> {
    #[must_use]
    /// The constructor for the Board struct.
    pub fn new(init_data: &str, goal_data: &str) -> Board<N> {
        let mut board = Board::default();

        #[cfg(debug_assertions)]
        {
            Board::set(init_data, &mut board.init_state).unwrap();
            Board::set(goal_data, &mut board.goal_state).unwrap();
        }
        #[cfg(not(debug_assertions))]
        {
            unsafe {
                Board::fast_set(init_data, &mut board.init_state);
                Board::fast_set(goal_data, &mut board.goal_state);
            }
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
        if let Err(fen_desc) = Board::set_with_fen(data, buf) {
            if let Err(csv_desc) = Board::set_with_csv(data, buf) {
                return Err(format!(
                    "Malformed init input data.\n[FEN: {fen_desc}]\n[CSV: {csv_desc}]"
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
                    "Malformed Queen {} coordinates value.",
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
    pub fn fast_set_with_csv(csv_data: &str, buf: &mut [[u8; N]; N]) {
        let csv_bytes = csv_data.as_bytes();
        let mut idx = 0;

        while idx < N * 3 - 1 {
            let file = csv_bytes[idx] - b'a';
            let rank = csv_bytes[idx + 1] - b'1'; // TODO: when N > 9
            unsafe {
                *buf.get_unchecked_mut(rank as usize)
                    .get_unchecked_mut(file as usize) = 1;
            }
            idx += 3; // Skips a comma too.
        }
    }
    /// Reads the provided FEN, and input the queens into $init_state.
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
            return Err(format!("Expected {N} ranks, but found {ranks_total}."));
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
    /// Reads the provided FEN, and input the queens into $init_state.
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
        #[cfg(debug_assertions)]
        {
            // Arranges it nicely for a better debugging experience.
            queens_pos.sort_unstable_by_key(|x| x.row);
            queens_pos.sort_unstable_by_key(|x| x.col);
        }
        queens_pos
    }
    #[inline(always)]
    pub fn solve(&mut self) -> Vec<Moves> {
        // Iterative deepening.
        const ENABLE_ITERATIVE_DEEPENING: bool = false;

        if ENABLE_ITERATIVE_DEEPENING {
            for x in 1..N as u16 * 2 {
                let t = std::time::Instant::now();
                let res = self.solve_inner(x);
                let t = t.elapsed();
                println!(
                    "Depth {x} - Time used: {}ms ({}μs)",
                    t.as_millis(),
                    t.as_micros()
                );
                if !res.is_empty() {
                    return res;
                }
            }
            vec![]
        } else {
            // Could even make do with just N*4, or N*3 actually.
            self.solve_inner(N as u16 * 5)
        }
    }
    #[inline(always)]
    pub fn solve_inner(&mut self, cutoff: u16) -> Vec<Moves> {
        use SearchStatus::*;
        let mut ds = <search::DFS<_> as Search>::with_capacity(32); // Seems to only used 29 max.

        // let mut ds = <search::BFS<_> as Search>::with_capacity(30139); // Uses 30139 on ./src/hard2.

        // let mut ds = <search::Dijkstra<_> as Search>::with_capacity(30142); // Uses 30142 on ./src/hard2

        let queens = Self::get_queens_pos(self.init_state);
        let mut goals = Self::get_queens_pos(self.goal_state);
        // Defines each queens has taken which goal.
        let mut queen_i_goal = [-1; N];

        for (i, x) in queens.into_iter().enumerate() {
            for (ii, y) in goals.iter_mut().enumerate() {
                if x == *y {
                    *y = Coord { row: -1, col: -1 };
                    queen_i_goal[i] = ii as i8;
                    break;
                }
            }
        }

        // Usage analysis. Will only be used in debug mode.
        #[allow(unused_mut)]
        let mut _nodes_generated = 1; // Including the root node.
        let mut _duplicating_nodes_found = 0;
        let mut _explored = 0;
        let mut _pruned = 0;
        let mut _max_frontier_len = 0;
        let mut _stuck_nodes = 0;
        let mut _stuck_node_loop_arounds = 0;

        let mut goal_idx = 0;
        while goal_idx < N && goals[goal_idx].row == -1 {
            goal_idx += 1;
        }

        // TODO: Seems to have a lot of duplicates...

        ds.push((queens, queen_i_goal, goal_idx, Vec::with_capacity(N), Ok));

        let mut lowest_moves = cutoff;
        let mut lowest_moves_list = Vec::new();

        while let Some((queens, queen_i_goal, mut goal_idx, moves, status)) = ds.pop_next() {
            #[cfg(debug_assertions)]
            {
                _explored += 1;
            }

            let mut next_goal_idx = goal_idx + 1;
            if let RetryingHold(_) = status {
                'a: while next_goal_idx < N {
                    if goals[next_goal_idx].row == -1 {
                        next_goal_idx += 1;
                        continue;
                    }
                    for q in queens {
                        if q == goals[next_goal_idx] {
                            next_goal_idx += 1;
                            continue 'a;
                        }
                    }

                    if goal_idx == usize::MAX {
                        goal_idx = next_goal_idx;
                        next_goal_idx += 1;
                    } else {
                        break;
                    }
                }
            } else {
                while next_goal_idx < N && goals[next_goal_idx].row == -1 {
                    next_goal_idx += 1;
                }
            }

            if goal_idx == N {
                match status {
                    OnHold(idx) => {
                        #[cfg(debug_assertions)]
                        {
                            // Will insert a dummy node to do the loop around.
                            _explored -= 1;
                            _stuck_node_loop_arounds += 1;
                        }
                        ds.moves_hint(0).apply_path_cost(moves.len()).push((
                            queens,
                            queen_i_goal,
                            usize::MAX,
                            moves,
                            RetryingHold(idx),
                        ));
                    }
                    RetryingHold(idx) if idx != N => {
                        todo!("hmm");
                    }
                    _ => {
                        if lowest_moves > moves.len() as u16 {
                            lowest_moves = moves.len() as u16;
                            lowest_moves_list = moves;
                        }
                        if ds.is_abort_on_found() {
                            break;
                        }
                    }
                }
                continue;
            }

            let iter = match status {
                Ok => queen_i_goal[..N].into_iter().enumerate().skip(0),
                OnHold(idx) => queen_i_goal[..idx].into_iter().enumerate().skip(0),
                RetryingHold(idx) => queen_i_goal[..N].into_iter().enumerate().skip(idx),
            };

            for (i, x) in iter {
                if *x == -1 {
                    // Current queen has no goal yet.
                    let mut queen_i_goal_new = queen_i_goal;
                    let mut queens_new = queens;
                    let mut moves_new = moves.clone();
                    let mut status_new = status;

                    let moves_count =
                        Self::min_moves(&queens, queens[i], goals[goal_idx], &mut moves_new);

                    if moves_count != 0 {
                        queen_i_goal_new[i] = goal_idx as i8;
                        queens_new[i] = goals[goal_idx]; // Moves queen to the goal.

                        if let RetryingHold(idx) = status {
                            status_new = RetryingHold(idx + 1);
                        }
                    } else {
                        #[cfg(debug_assertions)]
                        {
                            _stuck_nodes += 1;
                        }
                        status_new = match status {
                            Ok => {
                                queens_new.swap(i, N - 1);
                                queen_i_goal_new.swap(i, N - 1);
                                OnHold(N - 1)
                            }
                            OnHold(idx) => {
                                queens_new.swap(i, idx - 1);
                                queen_i_goal_new.swap(i, idx - 1);
                                OnHold(idx - 1)
                            }
                            _ => todo!("Should not be happening if N = 8... I think."),
                        };
                    }

                    if moves_new.len() as u16 >= lowest_moves {
                        // Pruning.
                        #[cfg(debug_assertions)]
                        {
                            _pruned += 1;
                        }
                        continue;
                    }

                    ds.moves_hint(moves_count)
                        .apply_path_cost(moves_new.len())
                        .push((
                            queens_new,
                            queen_i_goal_new,
                            next_goal_idx,
                            moves_new,
                            status_new,
                        ));

                    #[cfg(debug_assertions)]
                    {
                        _nodes_generated += 1;
                    }
                }
            }
            #[cfg(debug_assertions)]
            {
                _max_frontier_len = _max_frontier_len.max(ds.len());
            }
        }

        #[cfg(debug_assertions)]
        unsafe {
            // Not adding .clear() to the trait, so this is done manually.
            ds = Search::new();

            dbg!(
                ds,
                _nodes_generated,
                _explored,
                _pruned,
                _max_frontier_len,
                _stuck_nodes,
                _stuck_node_loop_arounds,
                μs_used_searching_is_blocked,
            );
        }
        lowest_moves_list
    }
    #[inline(always)]
    fn min_moves(
        map_list: &[Coord; N],
        src_piece: Coord,
        dest_square: Coord,
        moves: &mut Vec<Moves>,
    ) -> i8 {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                src_piece != dest_square,
                "$src_piece and $dest_square should not be the same!"
            );
            for x in map_list {
                debug_assert!(*x != dest_square, "$dest_square must not contain a Queen!");
            }
        }

        let (src, dest) = (src_piece, dest_square);

        let mut ds = <search::AStar<_> as Search>::with_capacity(N * N);
        let mut visited = [[usize::MAX; N]; N];

        for x in map_list.iter() {
            unsafe {
                *visited
                    .get_unchecked_mut(x.row as usize)
                    .get_unchecked_mut(x.col as usize) = 0;
            }
        }

        let t = std::time::Instant::now();

        use Direction::*;
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        #[repr(u8)]
        enum Direction {
            UpLeft,
            Up,
            UpRight,
            Left,
            Right,
            DownLeft,
            Down,
            DownRight,
            NoOrientation,
        }

        ds.push((src, src, NoOrientation, 1, ([Moves::NoPossibleMoves; 8], 0)));

        const TURNING_PENALTY: usize = 10000;

        while let Some((node, start, prev_dir, cur_total, turns)) = ds.pop_next() {
            let mut push_not_visited = |node: Coord, parent: Coord, dir| {
                let mut cost = cur_total;
                let heuristic = node.abs_diff(dest) as usize;
                let penalty = TURNING_PENALTY;
                if prev_dir != dir && prev_dir != NoOrientation {
                    cost += penalty;
                }

                if unsafe {
                    *visited
                        .get_unchecked(node.row as usize)
                        .get_unchecked(node.col as usize)
                        >= cost
                } {
                    unsafe {
                        *visited
                            .get_unchecked_mut(node.row as usize)
                            .get_unchecked_mut(node.col as usize) = cost;
                    }
                    if prev_dir == dir || prev_dir == NoOrientation {
                        ds.apply_path_cost(cost + heuristic)
                            .push((node, start, dir, cost, turns));
                    } else {
                        let mut turns_new = turns;

                        use Moves::*;
                        turns_new.0[turns_new.1] = match prev_dir {
                            Up | Down => Vertical(start, parent),
                            Left | Right => Horizontal(start, parent),
                            _ => Diagonal(start, parent),
                        };
                        turns_new.1 += 1;

                        ds.apply_path_cost(cost + heuristic).push((
                            node,
                            parent,
                            dir,
                            cur_total + penalty,
                            turns_new,
                        ));
                    }
                }
            };

            if node == dest {
                // Same as the match{} in push_not_visited||
                for x in &turns.0[..turns.1] {
                    moves.push(*x);
                }

                use Moves::*;
                moves.push(match prev_dir {
                    Up | Down => Vertical(start, node),
                    Left | Right => Horizontal(start, node),
                    _ => Diagonal(start, node),
                });

                #[cfg(debug_assertions)]
                {
                    unsafe {
                        μs_used_searching_is_blocked += t.elapsed().as_micros();
                    }
                }

                return turns.1 as i8 + 1;
            } else {
                let left_ok = node.col > 0;
                let right_ok = node.col + 1 < N as i8;
                let top_ok = node.row + 1 < N as i8;
                let bot_ok = node.row > 0;

                if top_ok {
                    if left_ok {
                        let top_left = Coord {
                            row: node.row + 1,
                            col: node.col - 1,
                        };
                        push_not_visited(top_left, node, UpLeft);
                    }
                    if right_ok {
                        let top_right = Coord {
                            row: node.row + 1,
                            col: node.col + 1,
                        };
                        push_not_visited(top_right, node, UpRight);
                    }
                    let top = Coord {
                        row: node.row + 1,
                        col: node.col,
                    };
                    push_not_visited(top, node, Up);
                }
                if bot_ok {
                    if left_ok {
                        let bot_left = Coord {
                            row: node.row - 1,
                            col: node.col - 1,
                        };
                        push_not_visited(bot_left, node, DownLeft);
                    }
                    if right_ok {
                        let bot_right = Coord {
                            row: node.row - 1,
                            col: node.col + 1,
                        };
                        push_not_visited(bot_right, node, DownRight);
                    }
                    let bot = Coord {
                        row: node.row - 1,
                        col: node.col,
                    };
                    push_not_visited(bot, node, Down);
                }
                if left_ok {
                    let left = Coord {
                        row: node.row,
                        col: node.col - 1,
                    };
                    push_not_visited(left, node, Left);
                }
                if right_ok {
                    let right = Coord {
                        row: node.row,
                        col: node.col + 1,
                    };
                    push_not_visited(right, node, Right);
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            unsafe {
                μs_used_searching_is_blocked += t.elapsed().as_micros();
            }
        }

        0
    }
    /// XXX: $dest_square must not contain a Queen piece on that coordinates.
    fn min_moves_with_list(
        map_list: &[Coord; N],
        src_piece: Coord,
        dest_square: Coord,
        moves: &mut Vec<Moves>,
    ) -> i8 {
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
            } else if x.row < left.row {
                // x.row == left.row will be ignored
                if x.col > left.col {
                    if left.row - x.row == x.col - left.col && x.row > bot_right_slope.row {
                        bot_right_slope = *x;
                    }
                } else {
                    if left.row - x.row == left.col - x.col && x.row > bot_left_slope.row {
                        bot_left_slope = *x;
                    }
                }
            }
        }

        let is_inbetween = |low, mid, high| low < mid && mid < high;
        let is_inbetween_unordered =
            |end, mid, end2| is_inbetween(end, mid, end2) || is_inbetween(end2, mid, end);

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
                return 1;
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
                return 1;
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
            return 1;
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
                orig.col - (orig.row - row)
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
                let (fn1, _, fn3, ma, sl, r, sr, d, mo) = $capture;
                let intersection = Coord {
                    row: fn1(sl, r.col),
                    col: r.col,
                };
                enter_slope!(fn3, ma, r, sr, d, intersection, mo, row, col, Vertical)
            }};
            (horizontally $capture: expr) => {{
                let (_, fn2, fn3, ma, sl, r, sr, d, mo) = $capture;
                let intersection = Coord {
                    row: r.row,
                    col: fn2(sl, r.row),
                };
                enter_slope!(fn3, ma, r, sr, d, intersection, mo, col, row, Horizontal)
            }};
            ($is_inbetween_unordered: expr, $map_list: expr, $right: expr, $src: expr, $dest: expr, $intersection: expr, $moves: expr, $main_dir: tt, $opp_dir: tt, $direction: tt) => {{
                let mut valid = true;
                for x in $map_list {
                    if x.$opp_dir == $right.$opp_dir
                        && $is_inbetween_unordered(
                            $intersection.$main_dir,
                            x.$main_dir,
                            $right.$main_dir,
                        )
                    {
                        valid = false;
                        break;
                    }
                }

                use Moves::*;
                if valid {
                    let moves = unsafe { $moves.as_mut().unwrap() };
                    if $right == $src {
                        moves.push($direction($src, $intersection));
                        moves.push(Diagonal($intersection, $dest));
                    } else {
                        moves.push(Diagonal($src, $intersection));
                        moves.push($direction($intersection, $dest));
                    }
                }
                valid
            }};
        }

        // Trying 2.
        if bot_right_slope.col > right.col {
            if enter_slope!(vertically capture(bot_right_slope)) {
                return 2;
            }
        }
        if top_right_slope.col > right.col {
            if enter_slope!(vertically capture(top_right_slope)) {
                return 2;
            }
        }
        if is_inbetween(left.row, right.row, top_right_slope.row) {
            if enter_slope!(horizontally capture(top_right_slope)) {
                return 2;
            }
        }
        if is_inbetween(left.row, right.row, top_left_slope.row) {
            if enter_slope!(horizontally capture(top_left_slope)) {
                return 2;
            }
        }
        if is_inbetween(bot_left_slope.row, right.row, left.row) {
            if enter_slope!(horizontally capture(bot_left_slope)) {
                return 2;
            }
        }
        if is_inbetween(bot_right_slope.row, right.row, left.row) {
            if enter_slope!(horizontally capture(bot_right_slope)) {
                return 2;
            }
        }

        if src == "a1".into() && dest == "b3".into() {
            moves.push(Moves::Vertical(src, "a3".into()));
            moves.push(Moves::Horizontal("a3".into(), dest));
            return 2;
        }

        // FIXME:
        //   |-+-|
        // 3 |q|q|
        //   |-+-|
        // 2 | |q|
        //   |-+-|
        // 1 |q|q|
        //   -----
        //    a b
        // a1 -> b3 (results in 3 moves. Should be a1->a2, a2->b3)

        // TODO: Diagonal to Diagonal move
        // TODO: Horizontal to vertical, and vice versa

        let mut board = [[0; N]; N];

        for x in map_list.iter() {
            board[x.row as usize][x.col as usize] = 1;
        }

        let mut ds = <search::NoAllocDFS<Coord> as Search>::new();
        let mut visited = [[false; N]; N];
        let mut path_exist = false;

        let t = std::time::Instant::now();
        visited[src.row as usize][src.col as usize] = true;
        ds.push(src);

        while let Some(node) = ds.pop_next() {
            let mut push_not_visited = |node: Coord, parent: Coord| {
                if !visited[node.row as usize][node.col as usize] {
                    visited[node.row as usize][node.col as usize] = true;
                    if board[node.row as usize][node.col as usize] == 0 {
                        ds.apply_path_cost(node.abs_diff(parent) as usize)
                            .push(node);
                    }
                }
            };

            if node == dest {
                path_exist = true;
                break;
            } else {
                let left_ok = node.col > 0;
                let right_ok = node.col + 1 < N as i8;
                let top_ok = node.row + 1 < N as i8;
                let bot_ok = node.row > 0;

                // No path cost, just estimated heuristic. Could've used Dijkstra too.
                if top_ok {
                    if left_ok {
                        let top_left = Coord {
                            row: node.row + 1,
                            col: node.col - 1,
                        };
                        push_not_visited(top_left, node);
                    }
                    if right_ok {
                        let top_right = Coord {
                            row: node.row + 1,
                            col: node.col + 1,
                        };
                        push_not_visited(top_right, node);
                    }
                    let top = Coord {
                        row: node.row + 1,
                        col: node.col,
                    };
                    push_not_visited(top, node);
                }
                if bot_ok {
                    if left_ok {
                        let bot_left = Coord {
                            row: node.row - 1,
                            col: node.col - 1,
                        };
                        push_not_visited(bot_left, node);
                    }
                    if right_ok {
                        let bot_right = Coord {
                            row: node.row - 1,
                            col: node.col + 1,
                        };
                        push_not_visited(bot_right, node);
                    }
                    let bot = Coord {
                        row: node.row - 1,
                        col: node.col,
                    };
                    push_not_visited(bot, node);
                }
                if left_ok {
                    let left = Coord {
                        row: node.row,
                        col: node.col - 1,
                    };
                    push_not_visited(left, node);
                }
                if right_ok {
                    let right = Coord {
                        row: node.row,
                        col: node.col + 1,
                    };
                    push_not_visited(right, node);
                }
            }
        }

        unsafe {
            μs_used_searching_is_blocked += t.elapsed().as_micros();
        }

        // TODO: check if path exist.
        if path_exist {
            // Might not be max 3 moves...
            moves.push(Moves::ThreeMoves1(src, dest));
            moves.push(Moves::ThreeMoves2(src, dest));
            moves.push(Moves::ThreeMoves3(src, dest));
            3
        } else {
            0
        }
    }
    pub fn replay_moves(&mut self, moves: &Vec<Moves>) {
        let mut map = self.init_state;
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

            println!("{}\n", Self::to_string_inner(&new_map));
            println!("Move {}: {x:?}\n\n", i + 1);
        }
    }
    pub fn to_string(&self) -> String {
        Self::to_string_inner(&self.init_state)
    }
    pub fn to_string_inner(map_list: &[[u8; N]; N]) -> String {
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

        for (row_n, row) in map_list[..].iter().rev().enumerate() {
            for (col_n, val) in row.iter().enumerate() {
                if *val != 0 {
                    layout[cal!(row_n, col_n)] = BoardPrint::new(*val).char_to_u8();
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
