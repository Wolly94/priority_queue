use std::collections::VecDeque;
use crate::pqueue::{Key, PriorityQueue};

pub struct NaivePQueue<V> where V: Key {
    l: VecDeque<(<V as Key>::Key, V)>
}

impl<V> PriorityQueue<V> for NaivePQueue<V> where V: Key,
    <V as Key>::Key: PartialOrd {
    fn empty() -> Self { NaivePQueue::new() }
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
        match self.find_min() {
            Some(k2) if k >= *k2 => {
                let mut i = 0;
                while i < self.l.len() && self.l[i].0 <= k { i+=1; }
                self.l.insert(i, (k, other));
            }
            _ => { self.l.push_front((k, other)); },
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

impl<V> NaivePQueue<V> where V: Key {
    pub fn new() -> Self {
        return NaivePQueue { l: VecDeque::new() }
    }
    pub fn from(l: Vec<V>) -> Self {
        return NaivePQueue { l: VecDeque::from(l.into_iter().map(|v| (v.eval(), v)).collect::<Vec<(<V as Key>::Key, V)>>()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_queue() {
        let mut q = NaivePQueue::from(vec![1, 2, 4]);
        q.insert(5);
        q.insert(2);
        let mut p = NaivePQueue::from(vec![3, 3, 6]);
        q.merge(&mut p);
        let r = [1, 2, 2, 3, 3, 4, 5, 6];
        let mut i = 0;
        while let Some(v) = q.delete_min() {
            assert_eq!(v, r[i]);
            i += 1;
        }
    }
}
