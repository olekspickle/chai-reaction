use serde::{Serialize, Deserialize};


use bevy::prelude::*;
use crate::prelude::*;

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,animate_basic_sprite);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, Reflect)]
pub enum SpriteFrames {
    #[default]
    ONE,
    Basic(u32, f32),
}

impl SpriteFrames {
    pub fn frames(&self) -> u32 {
        match self {
            SpriteFrames::ONE => 1,
            SpriteFrames::Basic(count, _) => *count,
        }
    }
}

#[derive(Component)]
pub struct BasicSpriteAnimationController {
    pub frame_count: u32,
    pub current_frame: u32,
    pub timer: Timer,
}

pub fn animate_basic_sprite (
    mut controller_query: Query<(&mut BasicSpriteAnimationController, &MachinePartType, &Children)>,
    mut sprite_query: Query<&mut Sprite,With<MachineSprite>>,
    time: Res<Time>,
) {
    for (mut controller, part, children) in controller_query.iter_mut() {
        controller.timer.tick(time.delta());
        if controller.timer.just_finished() {
            controller.current_frame = (controller.current_frame + 1) % controller.frame_count;

            let frame = controller.current_frame + controller.frame_count * part.context.rotation_index;

            for child in children.iter() {
                if let Ok(mut sprite) = sprite_query.get_mut(child) {
                    if let Some(atlas) = sprite.texture_atlas.as_mut() {
                        atlas.index = frame as usize;
                    }
                }
            }
            
        }
    }
}