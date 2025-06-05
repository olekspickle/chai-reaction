use crate::trait_union;
use std::fmt::Debug;
use std::hash::Hash;

trait_union!(EasilyHashable, Debug + Clone + Copy + Eq + PartialEq + Hash);
