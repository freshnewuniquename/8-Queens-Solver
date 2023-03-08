use std::collections::{BinaryHeap, VecDeque};

#[allow(dead_code)]
pub trait Search {
    type DS;
    type Item;

    const ABORT_ON_FOUND: bool;
    const IS_INFORMED: bool;

    // &self is used to make the function a method instead of an associative function.
    fn is_abort_on_found(&self) -> bool {
        Self::ABORT_ON_FOUND
    }
    fn is_informed_search(&self) -> bool {
        Self::IS_INFORMED
    }
    fn new() -> Self::DS;
    fn with_capacity(n: usize) -> Self::DS;
    fn push(&mut self, item: Self::Item);
    fn pop_next(&mut self) -> Option<Self::Item>;
    fn next(&self) -> Option<&Self::Item>;
    /// This function is used to supply the value of the node in an informed search.
    ///
    /// This function will keep the value temporarily, and is consumed on the next .push().
    ///
    /// NOTE: Make sure to use this function before a .push(), or a panic will occur,
    ///       or will cause undefined behaviour on release mode.
    ///
    /// # Examples:
    ///
    /// ```
    /// dijkstra.apply_node_value(val).push(node);
    /// ```
    fn apply_node_value(&mut self, value: usize) -> &mut Self {
        self
    }
}

// Iterative Depth-first search.
impl<T> Search for Vec<T> {
    type Item = T;
    type DS = Vec<T>;

    const ABORT_ON_FOUND: bool = false;
    const IS_INFORMED: bool = false;

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
    const IS_INFORMED: bool = true;

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

// Dijkstra's algorithm
// impl<T> Search for BinaryHeap<T> {

// }
