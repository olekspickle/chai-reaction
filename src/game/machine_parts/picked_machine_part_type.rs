use crate::prelude::*;

#[derive(Resource, Debug, Default, PartialEq)]
pub enum PickingState {
    #[default]
    None,
    Placing(MachinePartType),
    Erasing,
}

pub struct PickedMachinePartTypePlugin;

impl Plugin for PickedMachinePartTypePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PickingState>();
        app.add_observer(rotate_preview);
        app.add_observer(flip_preview);
    }
}

fn flip_preview(
    _on_flip: Trigger<OnFlip>,
    mut picking_state: ResMut<PickingState>,
    machine_part_config_by_type: Res<MachinePartConfigByType>,
) {
    if let PickingState::Placing(ref mut part_type) = *picking_state {
        let context = &mut part_type.context;
        if let Some(config) = machine_part_config_by_type.0.get(&part_type.name) {
            if !config.texture_info.flippable {
                return;
            }

            context.flipped ^= true;

            let max = config.texture_info.rotations;
            let half_max = max / 2;

            if context.flipped {
                // Clamp to [half_max, max)
                if context.rotation_index < half_max {
                    context.rotation_index += half_max;
                }
            } else {
                // Clamp to [0, half_max)
                if context.rotation_index >= half_max {
                    context.rotation_index -= half_max;
                }
            }
        }
    }
}

fn rotate_preview(
    on_rotate: Trigger<OnRotate>,
    mut picking_state: ResMut<PickingState>,
    machine_part_config_by_type: Res<MachinePartConfigByType>,
) {
    if let PickingState::Placing(ref mut part_type) = *picking_state {
        let context = &mut part_type.context;
        if let Some(config) = machine_part_config_by_type.0.get(&part_type.name) {
            let mut max = config.texture_info.rotations as i32;
            if config.texture_info.flippable {
                max /= 2;
            }

            let mut new_index: i32 = context.rotation_index as i32;
            if context.flipped {
                new_index -= max
            }
            new_index += on_rotate.0;
            new_index %= max;

            if new_index < 0 {
                new_index += max;
            }
            if context.flipped {
                new_index += max;
            }

            context.rotation_index = new_index as u32;
        }
    }
}
