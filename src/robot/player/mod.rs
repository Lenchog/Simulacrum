use crate::prelude::*;
use crate::weapons::prelude::*;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;

pub mod input;
pub mod movement;

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
        ],
    ),
)]
struct PlayerCollider;

#[derive(Component, PartialEq, Default)]
#[require(
    Robot,
    TnuaController,
    Recoil,
    Health(500),
    Energy,
    RespawnPoint,
    TnuaGhostSensor,
    TnuaProximitySensor
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

pub fn update_grid_coords(q_player: Single<(&GlobalTransform, &mut GridCoords), With<Player>>) {
    let (transform, mut grid_coords) = q_player.into_inner();
    *grid_coords =
        translation_to_grid_coords(transform.translation().truncate(), IVec2::splat(128));
}
