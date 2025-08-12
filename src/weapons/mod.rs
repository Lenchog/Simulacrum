use crate::{prelude::*, weapons::prelude::*};
pub mod prelude {
    pub use crate::weapons::{
        attack::*,
        melee::*,
        ranged::{general_ranged::*, grappling_hook::*, rocket_launcher::*, shoot::*, *},
        *,
    };
}

pub mod attack;
mod melee;
mod ranged;
pub mod weapon_input;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                update_explosion_timer,
                aim_weapon,
                weapon_cooldown,
                swing_weapon,
                shoot,
                handle_grapple_hook,
                retract_hook,
            ),
        )
        .add_systems(FixedPreUpdate, unhook)
        .add_event::<Unhook>()
        .add_event::<ShootEvent>();
    }
}

#[derive(Component, Default, Clone)]
pub struct Damage(pub u32);

#[derive(Component)]
pub struct UseTime(Timer);
impl Default for UseTime {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(500), TimerMode::Once))
    }
}

#[derive(Component, Default)]
#[require(Hitbox, RigidBody::Dynamic,
    Collider::circle(50.0),
    CollisionLayers = CollisionLayers::new(
        PhysicsLayers::PlayerHitbox,
        [PhysicsLayers::Enemy, PhysicsLayers::Ground],
    )
)]
pub struct PlayerHitbox;

#[derive(Component, Default)]
#[require(Sensor, CooldownFinished(true))]
pub struct Weapon;

#[derive(Component)]
pub struct SwingRotation(f32);

#[derive(Component)]
pub struct Equipped;

#[derive(Component, Clone)]
pub struct EnergyCost(pub u32);
impl Default for EnergyCost {
    fn default() -> Self {
        Self(5)
    }
}

#[derive(Component, Default)]
#[require(Damage(10), CollisionEventsEnabled, Collider)]
pub struct Hitbox;

#[derive(Component)]
#[require(Transform, Visibility::Inherited)]
pub struct RotationCenter;

#[derive(Component)]
#[require(Visibility::Inherited, Transform::from_xyz(200.0, 0.0, 0.0))]
pub struct WeaponTip;

#[derive(Component)]
pub struct CooldownFinished(bool);

enum WeaponType {
    Sword,
    Gun,
    FastGun,
    PowerGun,
    RocketLauncher,
    GrappleHook,
}

pub fn aim_weapon(
    q_rotation_center: Single<(&mut Transform, Option<&SwingRotation>), With<RotationCenter>>,
    window: Single<&Window>,
) {
    let (mut transform, swing_rotation) = q_rotation_center.into_inner();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let cursor_pos_frac = cursor_pos / window.size();
    let cursor_pos_signed = cursor_pos_frac - Vec2::splat(0.5);
    let mouse_angle = cursor_pos_signed.y.atan2(cursor_pos_signed.x);
    // if the cursor is on the left, angles must be negative
    let left_mult = if (-PI / 2.0..PI / 2.0).contains(&mouse_angle) {
        1.
    } else {
        -1.
    };
    let angle = match swing_rotation {
        Some(rotation) => rotation.0 * left_mult + mouse_angle,
        None => mouse_angle,
    };

    transform.rotation = Quat::from_rotation_z(-angle);
}
