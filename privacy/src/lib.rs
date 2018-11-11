pub mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
            ::outermost::middle_secret_function() // no compile error!
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function(); // compile error!
    outermost::inside::inner_function(); // compile error!
    outermost::inside::secret_function(); // compile error!
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
