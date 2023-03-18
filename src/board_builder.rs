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
    pub fn set_init(mut self, init_data: &'a str) -> Self {
        self.init_data = init_data;
        self
    }
    pub fn trust(mut self, trustable: bool) -> Self {
        self.is_trustable = trustable;
        self
    }
    /// A pipe to allow conditional, and other statements to be used in a builder pattern.
    ///
    /// The provided argument (Board builder struct) must be returned.
    ///
    /// For simple conditional statements, consider using [`pipe_if`].
    ///
    /// # Examples:
    ///
    /// ```
    /// let trustworthy = Some(true);
    /// let b = BoardBuilder::new()
    ///     .pipe(|s| if let Some(trust) = trustworthy { s.trust(trust) } else { s })
    ///     .build();
    /// ```
    pub fn pipe(self, fun: impl FnOnce(Self) -> Self) -> Self {
        fun(self)
    }
    /// A pipe that evaluates the statement provided, and only executes the closure if true.
    ///
    /// For more flexibility, consider [`pipe`].
    ///
    /// # Examples:
    ///
    /// ```
    /// let trustworthy = true;
    /// let b = BoardBuilder::new()
    ///     .pipe_if(trustworthy, |s| s.trust(true))
    ///     .build();
    /// ```
    pub fn pipe_if(self, statement: bool, fun: impl FnOnce(Self) -> Self) -> Self {
        if statement {
            fun(self)
        } else {
            self
        }
    }
    pub fn data_type(mut self, data_type: InputDataType) -> Self {
        self.data_type = data_type;
        self
    }
    #[must_use]
    pub fn build(self) -> Result<Board<N>, String> {
        let mut init_state = [[0; N]; N];

        let set = |data, buf| {
            use self::InputDataType::*;

            if self.is_trustable {
                unsafe {
                    match self.data_type {
                        CSV => {
                            Board::fast_set_with_csv(data, buf);
                        }
                        FEN => {
                            Board::fast_set_with_fen(data, buf);
                        }
                        Unknown => {
                            Board::fast_set(data, buf);
                        }
                    };
                }
            } else if self.data_type == CSV {
                // Functions with side effects, so no return values.
                if let Err(desc) = Board::set_with_csv(data, buf) {
                    return Err(format!("Malformed CSV input - {}", desc));
                }
            } else if self.data_type == FEN {
                if let Err(desc) = Board::set_with_fen(data, buf) {
                    return Err(format!("Malformed FEN input - {}", desc));
                }
            } else {
                if let Err(fen_desc) = Board::set(data, buf) {
                    if let Err(csv_desc) = Board::set(data, buf) {
                        return Err(format!(
                            "Malformed input data.\n[FEN: {fen_desc}]\n[CSV: {csv_desc}]"
                        ));
                    }
                }
            }
            Ok(())
        };

        set(self.init_data, &mut init_state)?;

        Ok(Board::<N> { init_state })
    }
}
