use crate::prelude::*;
use crate::setup::AppState;
use crate::weapons::prelude::*;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_tnua::control_helpers::TnuaSimpleAirActionsCounter;

pub mod input;
pub mod movement;

#[derive(Component, Default)]
pub struct RespawnPoint(pub GridCoords);

#[derive(Component)]
pub struct Energy(pub u32);
impl Default for Energy {
    fn default() -> Self {
        Self(100)
    }
}

#[derive(Resource)]
pub struct MaxEnergy(pub u32);

#[derive(Bundle, LdtkEntity, Default)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

#[derive(Resource)]
pub struct EquippedWeapons {
    pub left: Option<Entity>,
    pub right: Option<Entity>,
}

#[derive(Component, PartialEq)]
#[require(
    RobotCollider,
    CollisionLayers = CollisionLayers::new(
        PhysicsLayers::Player,
        [
            PhysicsLayers::Ground,
            PhysicsLayers::Spikes,
            PhysicsLayers::EnemyHitbox,
            PhysicsLayers::Collectable,
        ],
    ),
)]
struct PlayerCollider;

#[derive(Component, PartialEq, Default)]
#[require(
    Robot,
    TnuaController,
    Recoil,
    Health(100),
    Energy,
    RespawnPoint,
    TnuaSimpleAirActionsCounter,
    TnuaGhostSensor,
    TnuaProximitySensor,
    Friction::ZERO
)]
pub struct Player;

#[hot]
pub fn add_player() -> impl Bundle {
    (
        Player,
        PlayerCollider,
        (
            RigidBody::Dynamic,
            children![(RotationCenter, children!(WeaponTip))],
        ),
    )
}

pub fn update_grid_coords(mut q_entities: Query<(&Transform, &mut GridCoords), With<Player>>) {
    for (transform, mut grid_coords) in q_entities.iter_mut() {
        *grid_coords =
            translation_to_grid_coords(transform.translation.truncate(), IVec2::splat(128));
    }
}

pub fn update_respawn(
    q_player: Single<(&mut RespawnPoint, &TnuaProximitySensor)>,
    q_transform: Query<&GlobalTransform, With<Respawnable>>,
) {
    let (mut respawn, proximity) = q_player.into_inner();
    if let Some(output) = &proximity.output
        && let Ok(point) = q_transform.get(output.entity)
    {
        let point = point.translation().truncate() + Vec2::ZERO.with_y(128.0);
        respawn.0 = translation_to_grid_coords(point, IVec2::splat(128));
    }
}

pub fn death(mut ev_death: EventReader<DeathEvent>, mut next_state: ResMut<NextState<AppState>>) {
    for _ in ev_death.read() {
        next_state.set(AppState::MainMenu);
    }
}
