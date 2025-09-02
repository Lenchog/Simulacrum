use crate::{prelude::*, weapons::prelude::*};

pub fn sword(asset_server: &AssetServer, tip_entity: Entity) -> impl Bundle {
    MeleeWeaponBuilder {
        sprite: Sprite::from_image(asset_server.load("placeholder_bullet.png")),
        collider: Collider::rectangle(200.0, 50.0),
        damage: Damage(30),
    }
    .build(tip_entity)
}

#[derive(Component)]
struct MeleeWeaponBuilder {
    pub collider: Collider,
    pub sprite: Sprite,
    pub damage: Damage,
}

#[derive(Component)]
#[require(
    Weapon,
    Swingable,
    PlayerHitbox,
    Visibility::Hidden,
    CollisionLayers::new(PhysicsLayers::PlayerHitbox, PhysicsLayers::Enemy),
    ColliderDisabled
)]
pub struct MeleeWeapon;

impl MeleeWeaponBuilder {
    fn build(self, tip_entity: Entity) -> impl Bundle {
        (
            ChildOf(tip_entity),
            MeleeWeapon,
            self.sprite,
            self.collider,
            self.damage,
        )
    }
}

#[hot]
pub fn swing_weapon(
    q_rotation_center: Single<(Entity, &mut SwingRotation), With<RotationCenter>>,
    q_weapon: Single<(Entity, &mut CooldownFinished), With<Equipped>>,
    time: Res<Time<Fixed>>,
    mut commands: Commands,
) {
    let (rotation_center, mut rotation_offset) = q_rotation_center.into_inner();
    const SPEED: f32 = 0.15;
    rotation_offset.0 += SPEED * time.delta_secs() * 60.0;

    let (weapon, mut cooldown_finished) = q_weapon.into_inner();
    if rotation_offset.0 > 2.0 * PI {
        commands.entity(rotation_center).remove::<SwingRotation>();
        commands.entity(weapon).insert(ColliderDisabled);
        commands.entity(weapon).insert(Visibility::Hidden);
        *cooldown_finished = CooldownFinished(true);
        return;
    }
}

#[derive(Component, Default)]
pub struct Swingable;
