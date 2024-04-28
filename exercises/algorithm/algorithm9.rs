/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // https://zhuanlan.zhihu.com/p/187618450, swim and sink to impl a binary heap
    pub fn add(&mut self, value: T) {
        // insert on the tail
        self.items.push(value);
        self.count += 1;

        // swim from the tail
        let mut i = self.count;
        while i > 1 && // DeBug: not 0
            (self.comparator)(&self.items[i], &self.items[self.parent_idx(i)]){
            let parent_i = self.parent_idx(i);
            self.items.swap(i, parent_i);
            i = parent_i;
        }
    }

    pub fn delete(&mut self, idx: usize) -> Option<T> {
        // swap with the tail and remove it
        if self.count == 0 {
            return None;
        }
        let deleted_item = Some(self.items.swap_remove(idx));
        self.count -= 1;

        // sink from the head
        if self.count == 0 {
            return deleted_item;
        }
        let mut i = 1;
        while self.children_present(i){
            let small_child_i = self.smallest_child_idx(i);
            if !(self.comparator)(&self.items[i], &self.items[small_child_i]){
                self.items.swap(i, small_child_i);
            }
            i = small_child_i;
        }
        
        deleted_item
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
        let l_idx = self.left_child_idx(idx);
        let r_idx = self.right_child_idx(idx);
        if r_idx > self.count { // only have left_child
            l_idx
        } else if (self.comparator)
                        (&self.items[l_idx], &self.items[r_idx]){
            l_idx
        } else {
            r_idx
        }
    }
    
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // delete the first item of items (consume it)
		self.delete(1)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
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