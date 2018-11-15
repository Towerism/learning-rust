use std::ops::Deref;

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
fn simple_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[test]
fn use_box_like_reference() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[test]
fn use_my_box_like_reference() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[test]
fn simple_deref_coercion() {
    let m = MyBox::new(String::from("Rust"));
    assert_eq!(
        "Hello, Rust!",
        hello(&m)
        // without deref coercion:
        // hello(&(*m)[..])
    );
}

fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}
