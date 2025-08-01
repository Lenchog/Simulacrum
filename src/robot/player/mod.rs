use crate::prelude::*;
use crate::weapons::prelude::*;

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
            PhysicsLayers::EnemyHitbox,
        ],
    ),
)]
struct PlayerCollider;

#[derive(Component, PartialEq, Default)]
#[require(
    Robot,
    Recoil,
    Health(500),
    DashCooldownFrames,
    CaiyoteFrames,
    DashFrames,
    Energy
)]
pub struct Player;

#[hot]
pub fn add_player() -> impl Bundle {
    (
        Player,
        (
            RigidBody::Dynamic,
            children![PlayerCollider, (RotationCenter, children!(WeaponTip))],
        ),
    )
}
