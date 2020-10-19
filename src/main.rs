//use priority_queue::*;
mod performance_test;
use performance_test::{get_test_pqueues, get_test_vec, perform_insert, TestPQueue};

fn main() {
    let v = get_test_pqueues();
    let t = get_test_vec(100, 100);
    let res = perform_insert(v, t);
    for qtest in res {
        println!("{}", qtest);
    }
}
