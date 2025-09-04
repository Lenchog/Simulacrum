use crate::prelude::*;
use crate::setup::AppState;
use crate::weapons::prelude::*;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_tnua::control_helpers::TnuaSimpleAirActionsCounter;

pub mod input;
pub mod movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (death, update_grid_coords, update_respawn))
            .add_event::<DeathEvent>();
    }
}
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
    Recoil,
    Health(100),
    Energy,
    RespawnPoint,
    TnuaSimpleAirActionsCounter
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

fn update_grid_coords(mut q_entities: Query<(&Transform, &mut GridCoords), With<Player>>) {
    for (transform, mut grid_coords) in q_entities.iter_mut() {
        *grid_coords =
            translation_to_grid_coords(transform.translation.truncate(), IVec2::splat(128));
    }
}

fn update_respawn(
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

fn death(
    mut ev_death: EventReader<DeathEvent>,
    q_player: Single<Entity, With<Player>>,
    mut commands: Commands,
    current_state: ResMut<State<AppState>>,
) {
    let mut player_death = false;
    for entity in ev_death.read() {
        if entity.0 == *q_player {
            player_death = true;
        } else {
            commands.entity(entity.0).despawn();
        }
    }
    if player_death {
        commands.entity(*q_player).despawn();
        menu(
            "YOU DIED",
            &[ButtonType::MainMenu, ButtonType::ExitGame],
            commands,
            current_state.get().clone(),
        );
    }
}
