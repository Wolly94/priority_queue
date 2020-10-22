use std::fmt::Display;
use std::time::Instant;
use rand::{distributions::Uniform, Rng};
use priority_queue::*;

pub fn get_test_vec(n: usize, upper_bound: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, upper_bound);
    (0..n).map(|_| rng.sample(&range)).collect()
}

type QueueVec = Vec<Box<dyn PriorityQueue<usize>>>;

pub struct TestPQueue {
    //q: Box<dyn PriorityQueue<usize>>,
    insertion_time: u128,
    removal_time: u128,
}

impl Display for TestPQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "insertion time: {} ms\nremoval time: {} ms", self.insertion_time, self.removal_time)
    }
}

pub fn test_queues(l: QueueVec, test_vec: Vec<usize>) -> Vec<TestPQueue> {
    let mut result: Vec<TestPQueue> = vec![];
    for mut bqueue in l.into_iter() {
        let copy_vec = test_vec.clone();
        let mut now = Instant::now();
        for value in copy_vec {
            (*bqueue).insert(value);
        }
        let insertion_time = now.elapsed().as_millis();
        now = Instant::now();
        while let Some(_) = (*bqueue).delete_min() {
            //println!("{}", v);
        }
        let tpq = TestPQueue { /*q: bqueue,*/ removal_time: now.elapsed().as_millis(), insertion_time: insertion_time };
        result.push(tpq);
    }
    result
}
