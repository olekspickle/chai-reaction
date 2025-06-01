use crate::prelude::*;

pub fn remove_component_notify_on_fail<T: Component>(
    entity_to_remove_from: Entity,
    commands: &mut Commands,
) {
    if let Ok(mut entity_commands) = commands.get_entity(entity_to_remove_from) {
        entity_commands.remove::<T>();
    } else {
        warn!("{}", EntityError::CommandsCouldntGetEntity(&format!(
                "with component: {:?} (component removal attempt)",
                stringify!(T.type_name).to_string()
            )))
    }
}
