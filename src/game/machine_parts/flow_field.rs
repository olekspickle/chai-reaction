/*
  a flowfield is a type of sensor that applies a force to the sensed collider.
  thisbased on the red green values of flow texture.

*/

use bevy::prelude::*;
use avian2d::prelude::*;
use crate::prelude::{MachineSpriteInfo, Particle};

pub struct FlowFieldPlugin;
impl Plugin for FlowFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, do_flow_fields);
    }
}

#[derive(Component, Debug, Clone)]
#[require(Collider, Sensor)]
pub struct FlowField { 
    pub sprite_info: MachineSpriteInfo,
    pub rotation_index: u32,
}

/*
Create sensor equal to texture size
while colliding sensor, 
*/

fn do_flow_fields(
    mut flow_fields: Query<(Entity, &FlowField, &GlobalTransform)>,
    mut bodies: Query<(&mut ExternalImpulse, &GlobalTransform),With<Particle>>,
    collisions: Collisions,
    images: Res<Assets<Image>>,
    atlases: Res<Assets<TextureAtlasLayout>>,
) {
    for (flow_ent, flowfield, flow_transform) in &mut flow_fields {
        // Get the flowfield texture (vertical spritesheet)
        let image = if let Some(img) = images.get(&flowfield.sprite_info.image) {
            img
        } else {
            continue;
        };

        let mut rows = 1;
        if let Some(atlas) = flowfield
            .sprite_info
            .layout
            .as_ref()
            .and_then(|atlas| atlases.get(atlas)) 
        {
            rows = atlas.len() as u32;
        }
        
        let size = image.size() / UVec2::new(1,rows);
        let v_offset = size.y * flowfield.rotation_index;

        for contact_pair in collisions.collisions_with(flow_ent) {
            let other_ent = if (contact_pair.collider1 == flow_ent) {contact_pair.collider2} else {contact_pair.collider1} ;
            if let Ok((mut force, other_transform)) = bodies.get_mut(other_ent) {
                


                // Get relative position in flowfield local space
                let flow_pos = flow_transform.compute_matrix().inverse().transform_point3(other_transform.translation()).truncate();

                // convert to valid pixel position
                let pixel_pos = (flow_pos * vec2(1.0,-1.0) + 0.5 * size.as_vec2() ).as_ivec2();
                if pixel_pos.x < 0 || pixel_pos.y < 0 || pixel_pos.x >= size.x as i32 || pixel_pos.y >= size.y as i32 {
                    continue;
                }
                
                if let Ok(color) = image.get_color_at(pixel_pos.x as u32, pixel_pos.y as u32 + v_offset) {
                    let rgba = color.to_srgba();
                    let mut new_force = Vec2::new(
                        (rgba.red - 0.5) * 2.0, 
                        (rgba.green - 0.5) * 2.0
                    ) * rgba.alpha;

                    // Adjust vertical force multiplier based on sign
                    new_force.y *= if new_force.y > 0.0 { 500.0 } else { 5.0 };
                    new_force.x *= 50.0;

                    force.set_impulse(new_force);

                    // force.apply_impulse(new_force);
                }
            }
        }
    }
}