/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::cmp::Ordering;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Ord + std::clone::Clone,
{
    changed: bool,
    iter_count: usize,
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Ord + std::clone::Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            changed: false,
            iter_count: 0,
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 1
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.heapify_up(self.count);
    }

    pub fn heapify_up(&mut self, idx: usize) {
        let mut idx = idx;
        while self.parent_idx(idx) > 0 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                self.changed = true;
            }
            idx = parent_idx;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		1
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Ord + std::clone::Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Ord + std::clone::Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.changed {
            self.changed = false;
            self.iter_count = 0;
            if self.count > 1 {
                if (self.items[1] < self.items[2]) {
                    let slice = &mut self.items[1..];
                    slice.sort_by(|a, b| if a < b {Ordering::Less} else {Ordering::Greater});
                }
                else {
                    let slice = &mut self.items[1..];
                    slice.sort_by(|a, b| if a > b {Ordering::Less} else {Ordering::Greater});
                }
            }
        }
        if self.iter_count >= self.count {
            return None;
        }
        self.iter_count += 1;
        Some(self.items[self.iter_count].clone())
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}