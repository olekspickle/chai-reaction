use std::fmt::Debug;
use std::hash::Hash;
use crate::trait_union;

trait_union!(
    EasilyHashable,
    Debug + Clone + Copy + Eq + PartialEq + Hash
);