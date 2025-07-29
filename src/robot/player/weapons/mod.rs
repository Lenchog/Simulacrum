use crate::{
    WeaponOne,
    robot::{
        PhysicsLayers,
        player::{
            EquippedWeapons, Player,
            input::{
                SelectLeft, SelectRight, WeaponFive, WeaponFour, WeaponSix, WeaponThree, WeaponTwo,
            },
        },
    },
};
use std::time::Duration;

#[derive(Component, Default, Clone)]
pub struct Damage(pub u32);

#[derive(Component)]
#[require(Projectile)]
pub struct Despawnable;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Fired;

pub mod attack;

#[derive(Component)]
pub struct UseTime(pub Timer);
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
pub struct Equipped;

#[derive(Component, Default)]
pub struct Swingable;

#[derive(Component)]
pub struct MeleeWeaponBuilder {
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
    pub fn build(self, tip_entity: Entity) -> impl Bundle {
        (
            ChildOf(tip_entity),
            MeleeWeapon,
            self.sprite,
            self.collider,
            self.damage,
        )
    }
}
#[derive(Component, Default)]
pub struct Projectile;

#[derive(Component, Default)]
pub struct RangedWeaponBuilder {
    pub projectile_builder: ProjectileBuilder,
    pub sprite: Sprite,
    pub usetime: UseTime,
}
impl RangedWeaponBuilder {
    pub fn build(self, tip_entity: Entity) -> impl Bundle {
        (
            Weapon,
            ChildOf(tip_entity),
            self.usetime,
            self.projectile_builder,
            self.sprite,
        )
    }
}

#[derive(Component, Clone)]
pub struct EnergyCost(pub u32);
impl Default for EnergyCost {
    fn default() -> Self {
        Self(5)
    }
}

#[derive(Default, Component, Clone, Debug, PartialEq)]
pub enum ProjectileType {
    #[default]
    Normal,
    Rocket,
    Hook,
}

#[derive(Component, Clone, Default)]
pub struct ProjectileBuilder {
    pub speed: f32,
    pub gravity_scale: f32,
    pub sprite: Sprite,
    pub energy_cost: EnergyCost,
    pub damage: Damage,
    pub projectile_type: ProjectileType,
}
impl ProjectileBuilder {
    fn build(self, direction: Dir2) -> impl Bundle {
        (
            Projectile,
            Sensor,
            GravityScale(self.gravity_scale),
            LinearVelocity(*(direction) * self.speed),
            self.damage,
            self.sprite,
            self.energy_cost,
            self.projectile_type,
            PlayerHitbox,
        )
    }
}
#[derive(Component, Default)]
#[require(Damage(10), CollisionEventsEnabled, Collider)]
pub struct Hitbox;

#[derive(Component)]
#[require(Transform, Visibility::Inherited)]
pub struct RotationCenter;

#[derive(Component)]
pub struct SwingRotation(f32);

#[derive(Component)]
#[require(Visibility::Inherited, Transform::from_xyz(200.0, 0.0, 0.0))]
pub struct WeaponTip;

#[derive(Component)]
pub struct CooldownFinished(bool);

pub fn sword(asset_server: &AssetServer, tip_entity: Entity) -> impl Bundle {
    MeleeWeaponBuilder {
        sprite: Sprite::from_image(asset_server.load("placeholder_bullet.png")),
        collider: Collider::rectangle(200.0, 50.0),
        damage: Damage(50),
    }
    .build(tip_entity)
}

pub fn lazer_gun(asset_server: &AssetServer, tip_entity: Entity) -> impl Bundle {
    RangedWeaponBuilder {
        sprite: Sprite::from_image(asset_server.load("placeholder_gun.png")),
        projectile_builder: ProjectileBuilder {
            sprite: Sprite::from_image(asset_server.load("placeholder_bullet.png")),
            speed: 5000.0,
            ..default()
        },
        ..default()
    }
    .build(tip_entity)
}

pub fn faster_gun(asset_server: &AssetServer, tip_entity: Entity) -> impl Bundle {
    RangedWeaponBuilder {
        sprite: Sprite::from_image(asset_server.load("placeholder_gun.png")),
        usetime: UseTime(Timer::new(Duration::from_millis(300), TimerMode::Once)),
        projectile_builder: ProjectileBuilder {
            sprite: Sprite::from_image(asset_server.load("placeholder_bullet.png")),
            speed: 7000.0,
            ..default()
        },
    }
    .build(tip_entity)
}

pub fn power_gun(asset_server: &AssetServer, tip_entity: Entity) -> impl Bundle {
    RangedWeaponBuilder {
        sprite: Sprite::from_image(asset_server.load("placeholder_gun.png")),
        usetime: UseTime(Timer::new(Duration::from_millis(600), TimerMode::Once)),
        projectile_builder: ProjectileBuilder {
            sprite: Sprite::from_image(asset_server.load("placeholder_bullet.png")),
            speed: 3000.0,
            damage: Damage(30),
            ..default()
        },
    }
    .build(tip_entity)
}

#[derive(Component)]
pub struct Retracting;

#[derive(Component)]
pub struct Hooked;

#[derive(Component, Default)]
pub struct Hookable;

pub fn grappling_hook(asset_server: &AssetServer, tip_entity: Entity) -> impl Bundle {
    RangedWeaponBuilder {
        sprite: Sprite::from_image(asset_server.load("placeholder_gun.png")),
        usetime: UseTime(Timer::new(Duration::from_millis(600), TimerMode::Once)),
        projectile_builder: ProjectileBuilder {
            sprite: Sprite::from_image(asset_server.load("placeholder_bullet.png")),
            speed: 3000.0,
            damage: Damage(0),
            projectile_type: ProjectileType::Hook,
            ..default()
        },
    }
    .build(tip_entity)
}

pub fn handle_grapple_hook(
    q_projectile: Query<(Entity, &Transform, &ProjectileType)>,
    q_player: Single<&Transform, With<Player>>,
    q_hooked: Option<Single<(Entity, &mut LinearVelocity), With<Hooked>>>,
    mut commands: Commands,
) {
    let mut unhook = false;
    for (entity, transform, projectile_type) in q_projectile {
        if *projectile_type == ProjectileType::Hook {
            let distance = transform.translation.distance(q_player.translation);
            match distance {
                1000.0.. => {
                    commands.entity(entity).insert(Retracting);
                }
                ..100.0 => {
                    commands.entity(entity).despawn();
                    unhook = true;
                }
                _ => {}
            };
        }
    }
    if unhook && let Some(hooked) = q_hooked {
        let (entity, mut velocity) = hooked.into_inner();
        velocity.0 = Vec2::ZERO;
        commands.entity(entity).remove::<Hooked>();
    }
}

pub fn retract_hook(
    q_player: Single<(&GlobalTransform, &mut LinearVelocity), With<Player>>,
    q_hook: Single<(&mut LinearVelocity, &Transform), (With<Retracting>, Without<Player>)>,
    q_hooked: Query<
        Option<(&Hookable, &mut LinearVelocity)>,
        (With<Hooked>, Without<Retracting>, Without<Player>),
    >,
) {
    let (mut hook_velocity, hook_transform) = q_hook.into_inner();
    let (player_transform, mut player_velocity) = q_player.into_inner();
    let direction = (player_transform.translation() - hook_transform.translation)
        .truncate()
        .normalize();
    let hook_speed = hook_velocity.0.length();
    for hooked in q_hooked {
        if let Some((_, mut velocity)) = hooked {
            *velocity = *hook_velocity;
        } else {
            // pull player
            let direction = (hook_transform.translation - player_transform.translation())
                .truncate()
                .normalize();
            if player_velocity.0.length() < hook_velocity.0.length() {
                player_velocity.0 = hook_velocity.0;
            }
            player_velocity.0 = player_velocity.0.length() * direction;
            hook_velocity.0 = Vec2::ZERO;
            return;
        }
    }
    hook_velocity.0 = hook_speed * direction;
}

#[derive(Component)]
#[require(PlayerHitbox, Collider::circle(200.0), Damage(25), ExplosionTimer)]
pub struct Explosion;
pub fn explosion(asset_server: &AssetServer) -> impl Bundle {
    (
        Explosion,
        Sprite::from_image(asset_server.load("placeholder_robot.png")),
    )
}

#[derive(Component)]
pub struct ExplosionTimer(pub u8);
impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(60)
    }
}

pub fn update_explosion_timer(
    q_explosion: Query<(&mut ExplosionTimer, Entity), With<Explosion>>,
    mut commands: Commands,
) {
    for (mut timer, entity) in q_explosion {
        timer.0 = timer.0.saturating_sub(1);
        if timer.0 == 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn rocket_launcher(asset_server: &AssetServer, tip_entity: Entity) -> impl Bundle {
    RangedWeaponBuilder {
        sprite: Sprite::from_image(asset_server.load("placeholder_gun.png")),
        usetime: UseTime(Timer::new(Duration::from_millis(1500), TimerMode::Once)),
        projectile_builder: ProjectileBuilder {
            energy_cost: EnergyCost(20),
            sprite: Sprite::from_image(asset_server.load("placeholder_bullet.png")),
            speed: 3000.0,
            gravity_scale: 0.3,
            damage: Damage(100),
            projectile_type: ProjectileType::Rocket,
        },
    }
    .build(tip_entity)
}

#[derive(Resource)]
pub enum SelectedHand {
    Left,
    Right,
}

pub fn select_left(_: Trigger<Fired<SelectLeft>>, mut selected_hand: ResMut<SelectedHand>) {
    *selected_hand = SelectedHand::Left;
}
pub fn select_right(_: Trigger<Fired<SelectRight>>, mut selected_hand: ResMut<SelectedHand>) {
    *selected_hand = SelectedHand::Right;
}

pub enum WeaponType {
    Sword,
    Gun,
    FastGun,
    PowerGun,
    RocketLauncher,
    GrappleHook,
}

#[derive(Event)]
pub struct EquipEvent(WeaponType);

pub fn equip_sword(_: Trigger<Fired<WeaponOne>>, mut ev_weapon: EventWriter<EquipEvent>) {
    ev_weapon.write(EquipEvent(WeaponType::Sword));
}
pub fn equip_gun(_: Trigger<Fired<WeaponTwo>>, mut ev_weapon: EventWriter<EquipEvent>) {
    ev_weapon.write(EquipEvent(WeaponType::Gun));
}
pub fn equip_fast_gun(_: Trigger<Fired<WeaponThree>>, mut ev_weapon: EventWriter<EquipEvent>) {
    ev_weapon.write(EquipEvent(WeaponType::FastGun));
}
pub fn equip_power_gun(_: Trigger<Fired<WeaponFour>>, mut ev_weapon: EventWriter<EquipEvent>) {
    ev_weapon.write(EquipEvent(WeaponType::PowerGun));
}
pub fn equip_rocket_launcher(
    _: Trigger<Fired<WeaponFive>>,
    mut ev_weapon: EventWriter<EquipEvent>,
) {
    ev_weapon.write(EquipEvent(WeaponType::RocketLauncher));
}
pub fn equip_grappling_hook(_: Trigger<Fired<WeaponSix>>, mut ev_weapon: EventWriter<EquipEvent>) {
    ev_weapon.write(EquipEvent(WeaponType::GrappleHook));
}

pub fn equip_weapon(
    mut ev_weapon: EventReader<EquipEvent>,
    mut r_weapons: ResMut<EquippedWeapons>,
    tip_entity: Single<Entity, With<WeaponTip>>,
    asset_server: Res<AssetServer>,
    selected_hand: Res<SelectedHand>,
    mut commands: Commands,
) {
    for event in ev_weapon.read() {
        let entity = match event.0 {
            WeaponType::Sword => commands.spawn(sword(&asset_server, *tip_entity)).id(),
            WeaponType::Gun => commands.spawn(lazer_gun(&asset_server, *tip_entity)).id(),
            WeaponType::FastGun => commands.spawn(faster_gun(&asset_server, *tip_entity)).id(),
            WeaponType::PowerGun => commands.spawn(power_gun(&asset_server, *tip_entity)).id(),
            WeaponType::RocketLauncher => commands
                .spawn(rocket_launcher(&asset_server, *tip_entity))
                .id(),
            WeaponType::GrappleHook => commands
                .spawn(grappling_hook(&asset_server, *tip_entity))
                .id(),
        };
        let (selected_weapon, other_weapon) = match *selected_hand {
            SelectedHand::Left => (r_weapons.left, r_weapons.right),
            SelectedHand::Right => (r_weapons.right, r_weapons.left),
        };
        if let Some(equipped_weapon) = selected_weapon {
            commands.entity(equipped_weapon).despawn();
        }
        match *selected_hand {
            SelectedHand::Left => r_weapons.left = Some(entity),
            SelectedHand::Right => r_weapons.right = Some(entity),
        }
        if selected_weapon == other_weapon
            && let Some(other_weapon) = other_weapon
        {
            commands.entity(other_weapon).despawn()
        }
    }
}
