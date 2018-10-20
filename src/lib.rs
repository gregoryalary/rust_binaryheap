pub struct BinaryHeap {
    heap: Vec<i32>,
    size: usize
}

impl BinaryHeap {

    pub fn count(&self) -> usize {
        return self.heap.len();
    }

    pub fn add(&mut self, key : i32) {
        // todo
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.heap.pop() {
            None    => None,
            Some(x) => {
                self.size = self.size - 1;
                return if self.size > 0 {
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

    pub fn build_heap(&mut self) {
        for i in (0..(self.size / 2)).rev() {
            self.percolate_down(i);
        }
    }

    fn left(&self, i : usize) -> usize {
        return (i * 2) + 1;
    }

    fn right(&self, i : usize) -> usize {
        return self.left(i) + 1;
    }

    fn parent(&self, i : usize) -> usize {
        return (i - 1) / 2;
    }

    fn percolate_down(&mut self, index : usize) {

        let left = self.left(index);
        let right = self.right(index);
        let mut m = if left < self.size && self.heap[index] < self.heap[left] { left } else { index };
        m = if right < self.size && self.heap[m] < self.heap[right] { right } else { m };

        if m != index {
            let temp = self.heap[index];
            self.heap[index] = self.heap[m];
            self.heap[m] = temp;
            self.percolate_down(m);
        }
        
    }

    fn percolate_up(&mut self, index : usize) {
        // todo
    }

}

pub fn build_heap(keys : &[i32]) -> BinaryHeap {
    let mut b_heap = BinaryHeap {
        heap : keys.to_vec(),
        size : keys.len()
    };
    b_heap.build_heap();
    return b_heap;
}

#[cfg(test)]
mod tests {

    use super::*;

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
        println!("-> {:?}", heap.heap);
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

}