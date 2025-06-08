use super::*;

pub fn plugin(app: &mut App) {
    app.init_state::<PhysicsState>();
    app.add_plugins(
        PhysicsPlugins::default()
            .with_collision_hooks::<crate::game::conveyor_belts::ConveyorHooks>(),
    );
    app.add_systems(OnEnter(PhysicsState::Paused), pause);
    app.add_systems(OnEnter(Screen::Gameplay), start_paused);
    app.add_systems(OnEnter(PhysicsState::Running), run);
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PhysicsState {
    Paused,
    #[default]
    Running,
}

fn start_paused(mut next: ResMut<NextState<PhysicsState>>) {
    next.set(PhysicsState::Paused);
}

fn pause(mut time: ResMut<Time<Virtual>>, mut physics_time: ResMut<Time<Physics>>) {
    time.pause();
    physics_time.pause();
}

fn run(mut time: ResMut<Time<Virtual>>, mut physics_time: ResMut<Time<Physics>>) {
    time.unpause();
    physics_time.unpause();
}
