use crate::{MouseCoordinates, player::weapons::*};
use bevy_enhanced_input::prelude::*;

use crate::player::input::PrimaryAttack;

pub fn despawn_weapon(
    mut cooldown_finished: ResMut<CooldownFinished>,
    q_weapon: Single<&mut UseTime, With<Weapon>>,
    time: Res<Time>,
) {
    let mut timer = q_weapon.into_inner();
    timer.0.tick(time.delta());
    if timer.0.finished() {
        cooldown_finished.0 = true;
        timer.0.reset();
    }
}

pub fn shoot_projectile(
    _: Trigger<Fired<PrimaryAttack>>,
    mut commands: Commands,
    mut cooldown_finished: ResMut<CooldownFinished>,
    weapon_tip: Single<&GlobalTransform, With<WeaponTip>>,
    asset_server: Res<AssetServer>,
    mouse_coords: Res<MouseCoordinates>,
) {
    let weapon_translation = weapon_tip.clone().translation();
    let normalised_coords = mouse_coords.0 / (mouse_coords.0.x.abs() + mouse_coords.0.y.abs());
    if !cooldown_finished.0 {
        return;
    };
    cooldown_finished.0 = false;
    commands.spawn((
        ProjectileBundle {
            velocity: LinearVelocity(normalised_coords * 1000.0),
            ..default()
        },
        Sprite::from_image(asset_server.load("placeholder_bullet.png")),
        Transform::from_translation(weapon_translation),
    ));
}

pub fn aim_weapon(
    q_rotation_center: Single<(&GlobalTransform, &mut Transform), With<RotationCenter>>,
    mouse_coords: Res<MouseCoordinates>,
) {
    let (rotation_translation, mut rotation) = q_rotation_center.into_inner();
    let rotation_translation = rotation_translation.translation();
    let vec = Vec2 {
        x: rotation_translation.x,
        y: rotation_translation.y,
    };
    let cursor_angle = vec.angle_to(mouse_coords.0);

    rotation.rotation = Quat::from_rotation_z(cursor_angle);
}
