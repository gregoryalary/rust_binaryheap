pub mod binaryheap {

    pub struct BinaryHeap {
        heap: Vec<i32>
    }

    impl BinaryHeap {

        /**
         * Return the number of element in the heap
         */
        pub fn count(&self) -> usize {
            return self.heap.len();
        }

        /**
         * Add a new element into the heap
         */
        pub fn add(&mut self, key : i32) {
            self.heap.push(key);
            let last = self.heap.len() - 1;
            self.percolate_up(last);
        }

        /**
         * Remove the greatest element of the heap
         * Returns None if the heap is empty, or Some(x) otherwise
         */
        pub fn remove(&mut self) -> Option<i32> {
            match self.heap.pop() {
                None    => None,
                Some(x) => {
                    return if !self.heap.is_empty() {
                        let temp = self.heap[0];
                        self.heap[0] = x;
                        self.percolate_down(0);
                        Some(temp)
                    } else {
                        Some(x)
                    }
                } 
            }
        }

        fn left(&self, i : usize) -> usize {
            return (i * 2) + 1;
        }

        fn right(&self, i : usize) -> usize {
            return self.left(i) + 1;
        }

        fn parent(&self, i : usize) -> Option<usize> {
            return if i > 0 {
                Some((i - 1) / 2)
            } else {
                None
            }
        }

        fn percolate_down(&mut self, index : usize) {

            let left = self.left(index);
            let right = self.right(index);
            let mut m = if left < self.heap.len() && self.heap[index] < self.heap[left] { left } else { index };
            m = if right < self.heap.len() && self.heap[m] < self.heap[right] { right } else { m };

            if m != index {
                let temp = self.heap[index];
                self.heap[index] = self.heap[m];
                self.heap[m] = temp;
                self.percolate_down(m);
            }

        }

        fn percolate_up(&mut self, index : usize) {
            let opt_parent = self.parent(index);
            if let Some(parent) = opt_parent {
                if index > 0 && self.heap[parent] < self.heap[index] {
                    let temp = self.heap[parent];
                    self.heap[parent] = self.heap[index];
                    self.heap[index] = temp;
                    self.percolate_up(parent);
                }
            }
        }

        pub fn build_heap(&mut self) {
            for i in (0..(self.heap.len() / 2)).rev() {
                self.percolate_down(i);
            }
        }

    }

    /**
     * Returns a new binary heap built from the key
     * passed to the function
     */
    pub fn build_heap(keys : &[i32]) -> BinaryHeap {
        let mut b_heap = BinaryHeap {
            heap : keys.to_vec()
        };
        b_heap.build_heap();
        return b_heap;
    }

}

#[cfg(test)]
mod tests {

    use super::binaryheap::*;

    #[test]
    fn test_build_heap() {
        // Test 1
        let mut heap = build_heap(&[2, 1, 3]);
        assert_eq!(heap.remove(), Some(3));
        assert_eq!(heap.remove(), Some(2));
        assert_eq!(heap.remove(), Some(1));
        assert_eq!(heap.remove(), None);
        assert_eq!(heap.count(), 0);
        // Test 2
        let mut heap = build_heap(&[3, -1, 0, 10, 6, 4, 5]);
        assert_eq!(heap.remove(), Some(10));
        assert_eq!(heap.remove(), Some(6));
        assert_eq!(heap.remove(), Some(5));
        assert_eq!(heap.remove(), Some(4));
        assert_eq!(heap.remove(), Some(3)); 
        assert_eq!(heap.remove(), Some(0));
        assert_eq!(heap.remove(), Some(-1));
        assert_eq!(heap.remove(), None);
        assert_eq!(heap.count(), 0);
        // Test 3
        let mut heap = build_heap(&[]);
        assert_eq!(heap.remove(), None);
        assert_eq!(heap.count(), 0);
    }

    #[test]
    fn test_add() {
        // Test 1
        let mut heap = build_heap(&[]);
        heap.add(3);
        assert_eq!(heap.count(), 1);
        assert_eq!(heap.remove(), Some(3));
        heap.add(-1);
        heap.add(5);
        assert_eq!(heap.count(), 2);
        assert_eq!(heap.remove(), Some(5));
        assert_eq!(heap.remove(), Some(-1));
        assert_eq!(heap.remove(), None);
        assert_eq!(heap.count(), 0);
        // Test 2
        let mut heap = build_heap(&[3, -1, 0, 10]);
        heap.add(9);
        heap.add(13);
        assert_eq!(heap.count(), 6);
        assert_eq!(heap.remove(), Some(13));
        assert_eq!(heap.remove(), Some(10));
        assert_eq!(heap.remove(), Some(9));
        assert_eq!(heap.count(), 3);
    }

}