use std::collections::VecDeque;
use crate::pqueue::PriorityQueue;

pub struct NaivePQueue<V> where V: PartialOrd {
    l: VecDeque<V>,
}

impl<V> PriorityQueue<V> for NaivePQueue<V> where V: PartialOrd {
    fn empty() -> Self { NaivePQueue { l: VecDeque::new() } }
    fn find_min(&self) -> Option<&V> { self.l.get(0) }
    fn delete_min(&mut self) -> Option<V> { self.l.pop_front() }
    fn insert(&mut self, other: V) {
        let mut i = 0;
        while i < self.l.len() && self.l[i] <= other { i+=1; }
        self.l.insert(i, other);
    }
    fn merge(&mut self, other: &mut Self) {
        let mut i = 0;
        while let Some(v) = other.l.pop_front() {
            while i < self.l.len() && self.l[i] <= v { i+=1; }
            self.l.insert(i, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_queue() {
        let mut q: NaivePQueue<usize> = NaivePQueue::empty();
        q.insert(5);
        q.insert(2);
        let r = [2, 5];
        let mut i = 0;
        while let Some(v) = q.delete_min() {
            assert_eq!(v, r[i]);
            i += 1;
        }
    }
}
