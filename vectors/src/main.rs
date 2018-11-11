fn main() {
    let _v1 = vec![1, 2, 3];

    // we don't need to annotate Vec<T>
    // rust will infer its type based on
    // the values we push onto it.
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    for i in &mut v2 {
        *i += 50;
    }

    println!("v2 elements:");
    for i in &v2 {
        println!("{}", i);
    }
}
