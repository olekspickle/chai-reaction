use avian2d::{math::*, prelude::*};
use bevy::{
    ecs::system::{SystemParam, lifetimeless::Read},
    prelude::*,
};

pub fn plugin(_app: &mut App) {}

#[derive(Component)]
#[require(ActiveCollisionHooks::MODIFY_CONTACTS)]
pub struct ConveyorBelt {
    pub speed: f32,
}

// Define a custom `SystemParam` for our collision hooks.
// It can have read-only access to queries, resources, and other system parameters.
#[derive(SystemParam)]
pub struct ConveyorHooks<'w, 's> {
    conveyor_query: Query<'w, 's, (Read<ConveyorBelt>, Read<GlobalTransform>)>,
}

// Implement the `CollisionHooks` trait for our custom system parameter.
impl CollisionHooks for ConveyorHooks<'_, '_> {
    fn modify_contacts(&self, contacts: &mut ContactPair, _commands: &mut Commands) -> bool {
        let (entity1, entity2) = (contacts.collider1, contacts.collider2);

        println!("Number of manifolds: {}", contacts.manifolds.len());

        for (entity, direction) in [(entity1, -1.), (entity2, -1.)] {
            if let Ok((conveyor, transform)) = self.conveyor_query.get(entity) {
                
                for manifold in &mut contacts.manifolds {
                    manifold.tangent_speed += conveyor.speed * direction;
                }
            }
        }

        true


        // // Get the conveyor belt and its global transform.
        // // We don't know which entity is the conveyor belt, if any, so we need to check both.
        // // This also affects the sign used for the conveyor belt's speed to apply it in the correct direction.
        // let (Ok((conveyor_belt, _global_transform)), sign) = self
        //     .conveyor_query
        //     .get(contacts.collider1)
        //     .map_or((self.conveyor_query.get(contacts.collider2), -1.0), |q| {
        //         (Ok(q), -1.0)
        //     })
        // else {
        //     // If neither entity is a conveyor belt, return `true` early
        //     // to accept the contact pair without any modifications.
        //     return true;
        // };

        // // Iterate over all contact surfaces between the conveyor belt and the other collider,
        // // and apply a relative velocity to simulate the movement of the conveyor belt's surface.
        // for manifold in contacts.manifolds.iter_mut() {
        //     let tangent_velocity = sign * conveyor_belt.speed;
        //     manifold.tangent_speed = tangent_velocity.adjust_precision();
        // }

        // // Return `true` to accept the contact pair.
        // true
    }
}
