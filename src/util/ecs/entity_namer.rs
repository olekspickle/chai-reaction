use bevy::platform::collections::HashMap;
use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct Namer<T: EasilyHashable>(HashMap<T, Vec<u32>>);

impl<T: EasilyHashable> Namer<T> {
    pub fn make_name(&mut self, variant: T) -> String {
        let current_count_for_variant = self.0.entry(variant).or_default();
        format!(
            "{:?} {}",
            variant,
            Self::advance_number_string(current_count_for_variant)
        )
    }

    fn advance_number_string(variant_current_count: &mut Vec<u32>) -> String {
        let mut next_number_string = String::new();
        match variant_current_count.last_mut() {
            None => {
                variant_current_count.push(0);
            }
            Some(last_count) => {
                *last_count += 1;
                if *last_count == u32::MAX {
                    variant_current_count.push(0);
                }
            }
        }
        for count in variant_current_count.iter().rev() {
            next_number_string += &count.to_string();
        }
        next_number_string
    }
}

impl<T: EasilyHashable> Default for Namer<T> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}
