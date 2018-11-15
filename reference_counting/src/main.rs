use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

#[test]
fn reference_counting() {
    let mut rc_datapoints = Vec::new();
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    rc_datapoints.push(Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    rc_datapoints.push(Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        rc_datapoints.push(Rc::strong_count(&a));
    }
    rc_datapoints.push(Rc::strong_count(&a));

    assert_eq!(
        vec![1, 2, 3, 2],
        rc_datapoints
    )
}
