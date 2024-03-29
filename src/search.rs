use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, VecDeque};

#[allow(dead_code)]
pub trait Search {
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
    fn new() -> Self;
    fn with_capacity(n: usize) -> Self;
    fn next(&self) -> Option<&Self::Item>;
    fn pop_next(&mut self) -> Option<Self::Item>;
    fn push(&mut self, item: Self::Item);
    fn len(&self) -> usize;
    /// Functions as a hint for how many moves was made in that one search.
    ///
    /// This is useful for breadth-first search variant, where the search probes
    /// layer by layer.
    ///
    /// Other searches that does not require this function should just ignore
    /// this in their implementation.
    ///
    /// # Examples:
    ///
    /// ```
    /// bfs.moves_hint(moves).push(node);
    /// ```
    #[must_use]
    fn moves_hint(&mut self, _moves: i8) -> &mut Self {
        self
    }
    /// This function is used to supply the cost of the path in an search.
    ///
    /// This function will keep the value temporarily, and is used on the next
    /// .push().
    ///
    /// NOTE: Make sure to use this function before a .push(), or else it will
    ///       cause undefined behaviour.
    ///
    /// # Examples:
    ///
    /// ```
    /// dijkstra.apply_path_cost(cost).push(node);
    /// ```
    #[must_use]
    fn apply_path_cost(&mut self, _cost: usize) -> &mut Self {
        self
    }
    /// This function is used to supply the estimated heuristic for the current
    /// node in an information search.
    ///
    /// This function will only store the result of the computed heuristic
    /// that is performed outside of this function.
    ///
    /// # Examples:
    ///
    /// ```
    /// a_star.apply_path_cost(cost).apply_node_heuristic(est_cost).push(node);
    /// ```
    #[must_use]
    fn apply_node_heuristic(&mut self, _cost: usize) -> &mut Self {
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NoAllocDFS<T: Copy, const N: usize = 32>([T; N], usize);
impl<T: Copy, const N: usize> NoAllocDFS<T, N> {
    #[allow(dead_code)]
    pub fn to_vec(self) -> Vec<T> {
        self.0[..self.1].to_vec()
    }
}

impl<T: Copy, const N: usize> Search for NoAllocDFS<T, N> {
    type Item = T;

    const ABORT_ON_FOUND: bool = false;
    const IS_INFORMED: bool = false;

    fn new() -> Self {
        unsafe { NoAllocDFS([std::mem::MaybeUninit::zeroed().assume_init(); N], 0) }
    }
    fn with_capacity(_: usize) -> Self {
        unsafe { NoAllocDFS([std::mem::MaybeUninit::zeroed().assume_init(); N], 0) }
    }
    fn next(&self) -> Option<&Self::Item> {
        if self.1 > 0 {
            Some(&self.0[self.1 - 1])
        } else {
            None
        }
    }
    fn pop_next(&mut self) -> Option<Self::Item> {
        if self.1 > 0 {
            self.1 -= 1;
            Some(self.0[self.1])
        } else {
            None
        }
    }
    fn push(&mut self, item: Self::Item) {
        self.0[self.1] = item;
        self.1 += 1;
    }
    fn len(&self) -> usize {
        self.1
    }
}

// Iterative Depth-first search.
#[allow(dead_code)]
#[derive(Debug)]
pub struct DFS<T>(Vec<T>);
impl<T> Search for DFS<T> {
    type Item = T;

    const ABORT_ON_FOUND: bool = false;
    const IS_INFORMED: bool = false;

    fn new() -> Self {
        DFS(Vec::new())
    }
    fn with_capacity(n: usize) -> Self {
        DFS(Vec::with_capacity(n))
    }
    fn next(&self) -> Option<&Self::Item> {
        self.0.get(self.0.len() - 1)
    }
    fn pop_next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
    fn push(&mut self, item: Self::Item) {
        self.0.push(item);
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

// Breadth-first search.
#[derive(Debug)]
pub struct BFS<T>(VecDeque<(T, i8)>, i8);
impl<T> Search for BFS<T> {
    type Item = T;

    const ABORT_ON_FOUND: bool = true;
    const IS_INFORMED: bool = false;

    fn new() -> Self {
        BFS(VecDeque::new(), 0)
    }
    fn with_capacity(n: usize) -> Self {
        BFS(VecDeque::with_capacity(n), 0)
    }
    fn next(&self) -> Option<&Self::Item> {
        self.0.get(0).map(|x| &x.0)
    }
    fn pop_next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.0.pop_front() {
            if x.1 > 1 {
                self.0.push_back((x.0, x.1 - 1));
            } else {
                return Some(x.0);
            }
        }
        None
    }
    fn push(&mut self, item: Self::Item) {
        if self.1 == 0 {
            self.0.push_front((item, 0));
        } else {
            self.0.push_back((item, self.1));
        }
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn moves_hint(&mut self, moves: i8) -> &mut Self {
        self.1 = moves;
        self
    }
}

// Dijkstra's algorithm
#[derive(Debug)]
pub struct Dijkstra<T>(BinaryHeap<BinaryHeapItem<T>>, usize);
#[derive(Debug)]
pub struct BinaryHeapItem<T>(T, usize);

// Convert a max-heap to a min-heap.
impl<T> Ord for BinaryHeapItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}
impl<T> PartialOrd for BinaryHeapItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ord::cmp(&self, &other))
    }
}
impl<T> Eq for BinaryHeapItem<T> {}
impl<T> PartialEq for BinaryHeapItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<T> Search for Dijkstra<T> {
    type Item = T;

    const ABORT_ON_FOUND: bool = true;
    const IS_INFORMED: bool = false;

    fn new() -> Self {
        Dijkstra(BinaryHeap::new(), 0)
    }
    fn with_capacity(n: usize) -> Self {
        Dijkstra(BinaryHeap::with_capacity(n), 0)
    }
    fn next(&self) -> Option<&Self::Item> {
        self.0.peek().map(|x| &x.0)
    }
    fn pop_next(&mut self) -> Option<Self::Item> {
        self.0.pop().map(|x| x.0)
    }
    fn push(&mut self, item: Self::Item) {
        self.0.push(BinaryHeapItem(item, self.1));
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn apply_path_cost(&mut self, value: usize) -> &mut Self {
        self.1 = value;
        self
    }
}

// A*
#[derive(Debug)]
pub struct AStar<T>(BinaryHeap<BinaryHeapItem<T>>, usize);

impl<T> Search for AStar<T> {
    type Item = T;

    const ABORT_ON_FOUND: bool = true;
    const IS_INFORMED: bool = true;

    fn new() -> Self {
        Self(BinaryHeap::new(), 0)
    }
    fn with_capacity(n: usize) -> Self {
        Self(BinaryHeap::with_capacity(n), 0)
    }
    fn next(&self) -> Option<&Self::Item> {
        self.0.peek().map(|x| &x.0)
    }
    fn pop_next(&mut self) -> Option<Self::Item> {
        self.0.pop().map(|x| x.0)
    }
    fn push(&mut self, item: Self::Item) {
        self.0.push(BinaryHeapItem(item, self.1));
        self.1 = 0;
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn apply_path_cost(&mut self, cost: usize) -> &mut Self {
        self.1 += cost;
        self
    }
    fn apply_node_heuristic(&mut self, cost: usize) -> &mut Self {
        self.1 += cost;
        self
    }
}
