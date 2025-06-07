use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default().with_collision_hooks::<crate::game::conveyor_belts::ConveyorHooks>());
}
