use crate::prelude::*;

pub fn push_if_unique<T: PartialEq + Clone>(item: T, vec: &mut Vec<T>) -> ActionPerformed {
    let mut item_is_new = true;
    for vec_item in vec.clone() {
        if vec_item == item {
            item_is_new = false;
            break;
        }
    }
    if item_is_new {
        vec.push(item);
    }
    ActionPerformed(item_is_new)
}

pub fn swap_remove_by_value<T: PartialEq>(
    item_to_remove: &T,
    vec_to_remove_from: &mut Vec<T>,
) -> Option<T> {
    let optional_index_to_remove = item_to_index(item_to_remove, vec_to_remove_from);
    optional_index_to_remove.map(|index_to_remove| vec_to_remove_from.swap_remove(index_to_remove))
}

pub fn remove_by_value<T: PartialEq>(
    item_to_remove: &T,
    vec_to_remove_from: &mut Vec<T>,
) -> Option<T> {
    let optional_index_to_remove = item_to_index(item_to_remove, vec_to_remove_from);
    optional_index_to_remove.map(|index_to_remove| vec_to_remove_from.remove(index_to_remove))
}

pub fn item_to_index<T: PartialEq>(item_to_find: &T, vec_to_find_in: &[T]) -> Option<usize> {
    vec_to_find_in.iter().position(|x| *x == *item_to_find)
}

pub fn random_value<T: Clone>(vec_ref: &Vec<T>) -> Option<T> {
    random_index(vec_ref).map(|index| vec_ref[index].clone())
}

pub fn random_index<T>(vec_ref: &[T]) -> Option<usize> {
    RandomRange::new(0, vec_ref.len()).random()
}
