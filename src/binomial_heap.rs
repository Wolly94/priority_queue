use std::collections::VecDeque;
use crate::pqueue::PriorityQueue;

pub struct BinomialNode<V: PartialOrd> {
    order: usize,
    value: V,
    next_smaller_sibling: Option<Box<BinomialNode<V>>>,
    highest_order_child: Option<Box<BinomialNode<V>>>,
}

impl<V: PartialOrd> BinomialNode<V> {
    fn pretty_string(&self, depth: usize) -> String where V: std::fmt::Display {
        let mut s = self.info_string(depth)+"\n";
        let ochild = &self.highest_order_child;
        if let Some(child) = ochild {
            s += &(child.pretty_string(depth+1)+"\n");
        }
        if let Some(sibling) = &self.next_smaller_sibling {
            s += &(sibling.pretty_string(depth)+"\n");
        }
        s.pop();
        s
    }
    fn info_string(&self, depth: usize) -> String where V: std::fmt::Display {
        let s: String = "\t".repeat(depth)+"order: "+&self.order.to_string() + " value: " + &self.value.to_string();
        return s
    }
    fn take_child(&mut self) -> Option<BinomialNode<V>> {
        match self.highest_order_child.take() {
            None => None,
            Some(bn) => Some(*bn),
        }
    }
    fn take_sibling(&mut self) -> Option<BinomialNode<V>> {
        match self.next_smaller_sibling.take() {
            None => None,
            Some(bn) => Some(*bn),
        }
    }
    fn take_value(self) -> V { self.value }
    fn meld(mut self, mut other: Self) -> Self {
        if self.order == other.order {
            if self.value > other.value { return other.meld(self) }
            other.next_smaller_sibling = self.highest_order_child.take();
            self.highest_order_child = Some(Box::new(other));
            self.order += 1;
            return self
        } else { unreachable!(); }
    }
    fn new(v: V) -> Self {
        return BinomialNode { value: v, next_smaller_sibling: None, highest_order_child: None, order: 0 }
    }
}

pub struct BinomialHeap<V> where V: PartialOrd {
    roots: VecDeque<BinomialNode<V>>,
}

impl<V: PartialOrd> BinomialHeap<V> {
    fn get_min_index(&self) -> Option<usize> {
        if self.roots.len() == 0 { return None }
        let mut i = 0;
        let mut key_min = &self.roots[0].value;
        for (j, n) in self.roots.iter().enumerate() {
            if n.value < *key_min { key_min = &n.value; i = j; }
        }
        Some(i)
    }
}

impl<V: PartialOrd> PriorityQueue<V> for BinomialHeap<V> {
    fn empty() -> Self { BinomialHeap { roots: VecDeque::new() } }
    fn find_min(&self) -> Option<&V> {
        match self.get_min_index() {
            None => None,
            Some(i) => Some(&self.roots[i].value),
        }
    }
    fn delete_min(&mut self) -> Option<V> {
        if let Some(i) = self.get_min_index() {
            let mut min_node = self.roots.remove(i).unwrap();
            let mut ochild = min_node.take_child();
            let v = min_node.take_value();
            let mut other_roots = VecDeque::new();
            while let Some(mut child) = ochild {
                ochild = child.take_sibling();
                other_roots.push_front(child);
            }
            let mut other = BinomialHeap {roots: other_roots};
            self.merge(&mut other);
            Some(v)
        } else { None }
    }
    fn insert(&mut self, other: V) {
        let node = BinomialNode::new(other);
        let mut other = BinomialHeap { roots: VecDeque::from(vec![node]) };
        self.merge(&mut other);
    }
    fn merge(&mut self, other: &mut Self) {
        let mut r: VecDeque<BinomialNode<V>> = VecDeque::new();
        if other.roots.len() == 0 { return }
        if self.roots.len() == 0 { self.roots.append(&mut other.roots); return }
        let mut on1 = None;
        let mut on2 = None;
        let mut left_over: Option<BinomialNode<V>> = None;
        loop {
            if on1.is_none() { on1 = self.roots.pop_front(); }
            if on2.is_none() { on2 = other.roots.pop_front(); }
            match (on1.take(), on2.take(), left_over.take()) {
                (None, None, None) => {
                    self.roots = r;
                    return
                },
                (None, None, Some(n)) => { r.push_back(n) }
                (Some(n1), None, Some(n3)) if n1.order == n3.order => { left_over = Some(n1.meld(n3)); },
                (Some(n1), None, Some(n3)) => { on1 = Some(n1); r.push_back(n3); },
                (None, Some(n1), Some(n3)) if n1.order == n3.order => { left_over = Some(n1.meld(n3)); },
                (None, Some(n1), Some(n3)) => { on2 = Some(n1); r.push_back(n3); },
                (Some(n1), Some(n2), Some(n3)) if n1.order == n2.order && n1.order == n3.order => { left_over = Some(n2.meld(n3)); r.push_back(n1); },
                (Some(n1), Some(n2), Some(n3)) if n1.order == n3.order => { left_over = Some(n1.meld(n3)); on2 = Some(n2); },
                (Some(n1), Some(n2), Some(n3)) if n2.order == n3.order => { left_over = Some(n2.meld(n3)); on1 = Some(n1); },
                (Some(n1), Some(n2), Some(n3)) => { r.push_back(n3); on1 = Some(n1); on2 = Some(n2); },
                (Some(n), None, None) | (None, Some(n), None) => r.push_back(n),
                (Some(n1), Some(n2), None) if n1.order == n2.order => { left_over = Some(n1.meld(n2)); },
                (Some(n1), Some(n2), None) if n1.order < n2.order => { r.push_back(n1); on2 = Some(n2); },
                (Some(n1), Some(n2), None) if n1.order > n2.order => { r.push_back(n2); on1 = Some(n1); },

                _ => { unreachable!(); },
            }
        }
    }
}

impl<V: PartialOrd> std::fmt::Display for BinomialNode<V> where V: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_string(0))
    }
}

impl<V: PartialOrd> std::fmt::Display for BinomialHeap<V> where V: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "length: ".to_string() + &self.roots.len().to_string()+"\n";
        for v in self.roots.iter() {
            s += &(v.pretty_string(1)+"\n");
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_queue() {
        let mut q = BinomialHeap::new();
        q.insert(5);
        let r = vec![5];
        let mut i = 0;
        while let Some(v) = q.delete_min() {
            assert_eq!(v, r[i]);
            i += 1;
            if i >= r.len() { break }
        }
    }
}
