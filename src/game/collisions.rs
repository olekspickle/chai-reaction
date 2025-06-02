use avian2d::prelude::*;
use bevy::prelude::*;

// Define a new struct for your collision layers
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct GameCollisionLayers; // Marker type for collision layers

// Implement the Layer trait for your custom layers
impl Layer for GameCollisionLayers {
    // You can define constants for your layers here
    const DROPS: u32 = 0b01; // Layer 1 (for water droplets)
    const ENVIRONMENT: u32 = 0b10; // Layer 2 (for walls, floor, static objects)

    // Optional: Define ALL and NONE masks if needed, though not strictly required for this setup.
    const ALL: u32 = Self::DROPS | Self::ENVIRONMENT;
    const NONE: u32 = 0;

    // List all custom layers
    const CUSTOM_LAYERS: &'static [Self] = &[Self::DROPS, Self::ENVIRONMENT];
}
