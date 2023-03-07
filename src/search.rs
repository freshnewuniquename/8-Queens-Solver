use std::collections::VecDeque;

#[allow(dead_code)]
pub trait Search {
    type DS;
    type Item;

    const ABORT_ON_FOUND: bool;

    // &self is used to make the function a method instead of an associative function.
    fn is_abort_on_found(&self) -> bool {
        Self::ABORT_ON_FOUND
    }
    fn new() -> Self::DS;
    fn with_capacity(n: usize) -> Self::DS;
    fn push(&mut self, item: Self::Item);
    fn pop_next(&mut self) -> Option<Self::Item>;
    fn next(&self) -> Option<&Self::Item>;
}

// Iterative Depth-first search.
impl<T> Search for Vec<T> {
    type Item = T;
    type DS = Vec<T>;

    const ABORT_ON_FOUND: bool = false;

    fn new() -> Self::DS {
        Vec::new()
    }
    fn with_capacity(n: usize) -> Self::DS {
        Vec::with_capacity(n)
    }
    fn push(&mut self, item: Self::Item) {
        self.push(item);
    }
    fn pop_next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
    fn next(&self) -> Option<&Self::Item> {
        self.get(0)
    }
}

// Breadth-first search.
impl<T> Search for VecDeque<T> {
    type Item = T;
    type DS = VecDeque<T>;

    const ABORT_ON_FOUND: bool = true;

    fn new() -> Self::DS {
        VecDeque::new()
    }
    fn with_capacity(n: usize) -> Self::DS {
        VecDeque::with_capacity(n)
    }
    fn push(&mut self, item: Self::Item) {
        self.push_back(item);
    }
    fn pop_next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
    fn next(&self) -> Option<&Self::Item> {
        self.get(0)
    }
}
