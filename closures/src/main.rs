use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T, V>
    where V: std::hash::Hash + std::cmp::Eq,
          T: Fn(&V) -> V
{
    calculation: T,
    cache: HashMap<V, V>
}

impl<T, V> Cacher<T, V>
    where T: Fn(&V) -> V,
          V: std::hash::Hash + std::cmp::Eq
{
    fn new(calculation: T) -> Cacher<T, V> {
        Cacher {
            calculation,
            cache: HashMap::new()
        }
    }

    fn value(&mut self, arg: V) -> &V {
        let ref result;
        if self.cache.contains_key(&arg) {
            result = self.cache.get(&arg).unwrap();
        } else {
            let v = (self.calculation)(&arg);
            result = self.cache.entry(arg).or_insert(v);
        }
        result
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num: &u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num.clone()
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        )
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a: &u32| a.clone());
    {
        let _v1 = c.value(1);
    }
    let v2 = c.value(2);

    assert_eq!(*v2, 2);
}

#[test]
fn call_with_different_cacher_type() {
    let mut c = Cacher::new(|a: &String| a.clone());

    let v = c.value(String::from("asdf"));

    assert_eq!(*v, String::from("asdf"));
}
