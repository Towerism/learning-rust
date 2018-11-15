use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });
    print_strong_weak("leaf", &leaf);

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        print_strong_weak("branch", &branch);
        print_strong_weak("leaf", &leaf);
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    print_strong_weak("leaf", &leaf);
}

fn print_strong_weak(label: &str, node: &Rc<Node>) {
    println!(
        "{} strong = {}, weak {}",
        label,
        Rc::strong_count(&node),
        Rc::weak_count(&node)
    )
}
