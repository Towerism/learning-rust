pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0f64
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn averaging_empty_collection() {
        let items = AveragedCollection::new();

        assert_eq!(0f64, items.average());
    }

    #[test]
    fn averaging_one_value() {
        let mut items = AveragedCollection::new();

        items.add(1);

        assert_eq!(1f64, items.average());
    }

    #[test]
    fn averaging_many_values() {
        let mut items = AveragedCollection::new();

        items.add(3);
        items.add(5);
        items.add(7);

        assert_eq!(5f64, items.average());
    }
}
