pub trait Key {
    type Key: PartialOrd;
    fn eval(&self) -> Self::Key;
}

pub trait PriorityQueue<V> where V: Key {
    fn empty() -> Self where Self: Sized;
    fn find_min(&self) -> Option<&<V as Key>::Key>;
    fn delete_min(&mut self) -> Option<V>;
    fn insert(&mut self, other: V);
    fn merge(&mut self, other: &mut Self) where Self: Sized;
    fn check(&mut self) -> bool where <V as Key>::Key: std::fmt::Display {
        if let Some(v) = self.delete_min() {
            let mut old_key = v.eval();
            while let Some(v) = self.delete_min() {
                if old_key > v.eval() { 
                    println!("wrong order: {} {}", old_key, v.eval());
                    return false 
                }
                else { old_key = v.eval(); }
            }
        }
        true
    }
}

pub enum RPQ<V, Q>
where V: Key, Q: PriorityQueue<V> {
    Empty,
    Is(V, Q),
}

impl<V, Q> PriorityQueue<V> for RPQ<V, Q>
where V: Key+Copy, Q: PriorityQueue<V>+Copy {
    fn empty() -> Self { RPQ::Empty }
    fn find_min(&self) -> Option<&<V as Key>::Key> {
        match &self {
            RPQ::Empty => None,
            RPQ::Is(ref v, _) => Some(&v.eval()),
        }
        //todo!()
    }
    fn delete_min(&mut self) -> Option<V> {
        match self {
            RPQ::Empty => None,
            RPQ::Is(mut v, mut q) => {
                if let Some(new_min) = q.delete_min() {
                    Some(std::mem::replace(&mut v, new_min))
                } else {
                    *self = RPQ::Empty;
                    Some(v)
                }
            },
        }
    }
    fn insert(&mut self, other: V) {
        match self {
            RPQ::Empty => {*self = RPQ::Is(other, Q::empty()); },
            RPQ::Is(mut v, q) if other.eval() < v.eval() => { q.insert(std::mem::replace(&mut v, other)); },
            RPQ::Is(_v, q) => { q.insert(other); },
        }
    }
    fn merge(&mut self, other: &mut Self) {
        if let RPQ::Is(ref v2, ref mut q2) = other {
            match self {
                RPQ::Empty => *self = RPQ::Is(*v2, *q2),
                RPQ::Is(ref mut v1, mut q1) if v2.eval() < v1.eval() => {
                    q1.insert(std::mem::replace(v1, *v2));
                    q1.merge(q2);
                },
                RPQ::Is(_v1, mut q1) => {
                    q2.insert(*v2);
                    q1.merge(q2);
                },
            }
        }
    }
}

impl Key for usize {
    type Key = usize;
    fn eval(&self) -> Self::Key { *self }
}
