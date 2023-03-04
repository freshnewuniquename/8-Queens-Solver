use crate::board::Board;

#[derive(PartialEq, Eq)]
pub enum InputDataType {
    CSV,
    FEN,
    Unknown,
}

pub struct BoardBuilder<'a, const N: usize> {
    init_data: &'a str,
    is_trustable: bool,
    data_type: InputDataType,
}

#[allow(dead_code)]
impl<'a, const N: usize> BoardBuilder<'a, N> {
    #[must_use]
    pub fn new() -> BoardBuilder<'static, N> {
        BoardBuilder {
            init_data: "",
            is_trustable: false,
            data_type: InputDataType::Unknown,
        }
    }
    pub fn set(mut self, init_data: &'a str) -> Self {
        self.init_data = init_data;
        self
    }
    pub fn trust(mut self, trustable: bool) -> Self {
        self.is_trustable = trustable;
        self
    }
    pub fn data_type(mut self, data_type: InputDataType) -> Self {
        self.data_type = data_type;
        self
    }
    #[must_use]
    pub fn build(self) -> Result<Board<N>, String> {
        let mut board = Board::default();

        use self::InputDataType::*;

        if self.is_trustable {
            unsafe {
                match self.data_type {
                    CSV => board.fast_set_with_csv(self.init_data),
                    FEN => board.fast_set_with_fen(self.init_data),
                    Unknown => {
                        if self.init_data.as_bytes()[0].is_ascii_lowercase() {
                            board.fast_set_with_csv(self.init_data);
                        } else {
                            board.fast_set_with_fen(self.init_data);
                        }
                    }
                };
            }
        } else if self.data_type == CSV {
            // Functions with side effects, so no return values.
            if let Err(desc) = board.set_with_csv(self.init_data) {
                return Err(format!("Malformed CSV input - {}", desc));
            }
        } else if self.data_type == FEN {
            if let Err(desc) = board.set_with_fen(self.init_data) {
                return Err(format!("Malformed FEN input - {}", desc));
            }
        } else {
            if let Err(fen_desc) = board.set_with_fen(self.init_data) {
                if let Err(csv_desc) = board.set_with_csv(self.init_data) {
                    return Err(format!(
                        "Malformed input data.\n[FEN: {fen_desc}]\n[CSV: {csv_desc}]"
                    ));
                }
            }
        }
        Ok(board)
    }
}
