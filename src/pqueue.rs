pub trait Key {
    type Key: PartialOrd;
    fn eval(&self) -> Self::Key;
}

pub trait PriorityQueue<V> where V: PartialOrd {
    fn empty() -> Self where Self: Sized;
    fn find_min(&self) -> Option<&V>;
    fn delete_min(&mut self) -> Option<V>;
    fn insert(&mut self, other: V);
    fn merge(&mut self, other: &mut Self) where Self: Sized;

    // just to see if it actually does what it should
    fn check(&mut self) -> bool where V: std::fmt::Display {
        if let Some(p) = self.delete_min() {
            let mut old_key = p;
            while let Some(p2) = self.delete_min() {
                if old_key > p2 { 
                    println!("wrong order: {} {}", old_key, p2);
                    return false 
                }
                else { old_key = p2; }
            }
        }
        true
    }
}

// same as Q but implements delete_min in O(1)
pub enum RPQ<V, Q>
where V: PartialOrd, Q: PriorityQueue<V> {
    Empty,
    Is(V, Q),
}

impl<V, Q> PriorityQueue<V> for RPQ<V, Q>
where V: PartialOrd, Q: PriorityQueue<V> {
    fn empty() -> Self { RPQ::Empty }
    fn find_min(&self) -> Option<&V> {
        match self {
            RPQ::Empty => None,
            RPQ::Is(v, _) => Some(&v),
        }
    }
    fn delete_min(&mut self) -> Option<V> {
        match self {
            RPQ::Empty => return None,
            RPQ::Is(ref mut v, ref mut q) => {
                if let Some(new_min) = q.delete_min() {
                    return Some(std::mem::replace(v, new_min))
                }
            },
        }
        match std::mem::replace(self, RPQ::Empty) {
            RPQ::Is(v, _) => Some(v),
            _ => unreachable!(),
        }
        //match std::mem::replace(self, RPQ::Empty) {
        //    RPQ::Empty => None,
        //    RPQ::Is(v, mut q) => {
        //        if let Some(new_min) = q.delete_min() {
        //            std::mem::replace(self, RPQ::Is(new_min, q));
        //        }
        //        Some(v)
        //    },
        //}
    }
    fn insert(&mut self, other: V) {
        match self {
            RPQ::Empty => {*self = RPQ::Is(other, Q::empty()); },
            RPQ::Is(ref mut p, q) if other < *p => { q.insert(std::mem::replace(p, other)); },
            RPQ::Is(_, q) => { q.insert(other); },
        }
    }
    fn merge(&mut self, other: &mut Self) {
        if let RPQ::Is(v2, mut q2) = std::mem::replace(other, RPQ::Empty) {
            match self {
                RPQ::Empty => *self = RPQ::Is(v2, q2),
                RPQ::Is(ref mut v1, ref mut q1) if v2 < *v1 => {
                    q1.insert(std::mem::replace(v1, v2));
                    q1.merge(&mut q2);
                },
                RPQ::Is(_v1, ref mut q1) => {
                    q2.insert(v2);
                    q1.merge(&mut q2);
                },
            }
        }
    }
}
