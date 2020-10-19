use std::collections::VecDeque;
use crate::pqueue::{Key, PriorityQueue};

pub struct BisectionPQueue<V> where V: Key {
    l: VecDeque<(<V as Key>::Key, V)>
}

impl<V: Key> BisectionPQueue<V> {
    pub fn new() -> Self {
        BisectionPQueue { l: VecDeque::new() }
    }
}

impl<V> PriorityQueue<V> for BisectionPQueue<V> where V: Key,
    <V as Key>::Key: PartialOrd {
    fn empty() -> Self { BisectionPQueue::new() }
    fn find_min(&self) -> Option<&<V as Key>::Key> {
        match self.l.len() {
            0 => None,
            _ => Some(&self.l[0].0),
        }
    }
    fn delete_min(&mut self) -> Option<V> { 
        match self.l.pop_front() {
            None => None,
            Some(t) => Some(t.1),
        }
    }
    fn insert(&mut self, other: V) {
        let k = other.eval();
        if self.l.len() == 0 { self.l.push_back((k, other)); return }
        let mut i0 = 0;
        let mut i1 = self.l.len()-1;
        loop {
            let k1 = &self.l[i0].0;
            let k2 = &self.l[i1].0;
            if *k1 <= k && k < *k2 {
                let j = (i0+i1)/2;
                if j == i0 {
                    self.l.insert(i1, (k, other));
                    return
                }
                let kj = &self.l[j].0;
                if *kj <= k {
                    i0 = j;
                } else {
                    i1 = j;
                }
            }
            else if k < *k1 { self.l.push_front((k, other)); return }
            else { self.l.push_back((k, other)); return }
        }
    }
    fn merge(&mut self, other: &mut Self) {
        let mut i = 0;
        while let Some((k, v)) = other.l.pop_front() {
            while i < self.l.len() && self.l[i].0 <= k { i+=1; }
            self.l.insert(i, (k, v));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_queue() {
        let mut q = BisectionPQueue::new();
        for i in 0..10 { q.insert(20-2*i); }
        let mut p = BisectionPQueue::new();
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
