use rand::{Rng};

#[derive(Debug, Clone)]
pub struct RandomRange<T: PartialOrd + Copy + rand::distributions::uniform::SampleUniform> {
    pub min: T,
    pub max: T,
}

impl<T: PartialOrd + Copy + rand::distributions::uniform::SampleUniform> RandomRange<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    pub fn random(&self) -> Option<T> {
        if self.min < self.max {
            let mut random = rand::thread_rng();
            Some(random.gen_range(self.min..self.max))
        } else {
            None
        }
    }

    pub fn within_range(&self, value: T) -> bool {
        self.min <= value && value <= self.max
    }
}
