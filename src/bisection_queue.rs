use std::collections::VecDeque;
use crate::pqueue::PriorityQueue;

pub struct BisectionPQueue<V> where V: PartialOrd {
    l: VecDeque<V>,
}

impl<V> PriorityQueue<V> for BisectionPQueue<V> where V: PartialOrd {
    fn empty() -> Self { BisectionPQueue { l: VecDeque::new() } }
    fn find_min(&self) -> Option<&V> { self.l.get(0) }
    fn delete_min(&mut self) -> Option<V> { self.l.pop_front() }
    fn insert(&mut self, other: V) {
        if self.l.len() == 0 { self.l.push_back(other); return }
        let mut i0 = 0;
        let mut i1 = self.l.len()-1;
        loop {
            if self.l[i0] <= other && other < self.l[i1] {
                let j = (i0+i1)/2;
                if j == i0 {
                    self.l.insert(i1, other);
                    return
                }
                if self.l[j] <= other {
                    i0 = j;
                } else {
                    i1 = j;
                }
            }
            else if other < self.l[i0] { self.l.push_front(other); return }
            else { self.l.push_back(other); return }
        }
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
        let mut q = BisectionPQueue::empty();
        for i in 0..10 { q.insert(20-2*i); }
        let mut p = BisectionPQueue::empty();
        for j in 0..10 { q.insert(20-1-2*j); }
        q.merge(&mut p);
        let r = (1..21).collect::<Vec<usize>>();
        let mut i = 0;
        while let Some(v) = q.delete_min() {
            assert_eq!(v, r[i]);
            i += 1;
        }
    }
}
