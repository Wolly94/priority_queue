mod performance_test;
use priority_queue::*;
use performance_test::{get_test_vec, test_queues};

fn main() {
    let v: Vec<Box<dyn PriorityQueue<usize>>> = vec![
        //Box::new(NaivePQueue::empty()),
        //Box::new(BisectionPQueue::empty()),
        Box::new(BinomialHeap::empty()),
        Box::new(<RPQ::<usize, BinomialHeap<usize>> as PriorityQueue<_>>::empty()),
    ];
    let t = get_test_vec(1000000, 1000000);
    let res = test_queues(v, t);
    for qtest in res {
        println!("{}", qtest);
    }
}
