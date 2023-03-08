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
    fn new() -> Self::DS {
        Self::DS::new()
    }
    fn with_capacity(n: usize) -> Self::DS {
        Self::DS::with_capacity(n)
    }
    fn push(&mut self, item: Self::Item) {
        Self::DS.push(item)
    }
    fn pop_next(&mut self) -> Option<Self::Item> {
        Self::DS.pop()
    }
    fn next(&self) -> Option<&Self::Item> {
        Self::DS.get(0)
    }
    /// Functions as a hint for how many moves was made in that one search.
    ///
    /// This is useful for breadth-first search variant, where the search probes layer
    /// by layer.
    ///
    /// Other searches that does not require this function should just ignore this in
    /// their implementation.
    fn moves_hint(&mut self, _moves: i8) -> &mut Self {
        self
    }
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
    fn apply_node_value(&mut self, _value: usize) -> &mut Self {
        self
    }
}

// Iterative Depth-first search.
impl<T> Search for Vec<T> {
    type Item = T;
    type DS = Vec<T>;

    const ABORT_ON_FOUND: bool = false;
    const IS_INFORMED: bool = false;
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
