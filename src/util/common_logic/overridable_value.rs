/// saves an original value but can store a temporary alternative one
#[derive(Debug, Clone)]
pub struct OverridableValue<T: Copy> {
    value: T,
    overriding_value: Option<T>,
}

impl<T: Copy> OverridableValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            overriding_value: None,
        }
    }

    pub fn override_value(&mut self, value: T) {
        self.overriding_value = Some(value);
    }

    pub fn value(&self) -> T {
        self.overriding_value.unwrap_or(self.value)
    }

    pub fn reset(&mut self) {
        self.overriding_value = None;
    }
}
