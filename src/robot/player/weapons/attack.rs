use crate::{
    MouseCoordinates,
    robot::{
        health::get_hits,
        player::{input::PrimaryAttack, weapons::*},
    },
};
use bevy_enhanced_input::prelude::*;

pub fn weapon_cooldown(
    q_weapon: Query<(&mut UseTime, &mut CooldownFinished), With<Weapon>>,
    time: Res<Time>,
) {
    for (mut timer, mut cooldown_finished) in q_weapon {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            *cooldown_finished = CooldownFinished(true);
            timer.0.reset();
        }
    }
}

pub fn shoot_projectile(
    _: Trigger<Fired<PrimaryAttack>>,
    mut commands: Commands,
    mut q_weapon: Query<
        (
            &Transform,
            &mut CooldownFinished,
            Option<&ProjectileBuilder>,
        ),
        With<Weapon>,
    >,
    q_tip_transform: Single<&GlobalTransform, With<WeaponTip>>,
    res_equipped_weapons: Res<EquippedWeapons>,
    mouse_coords: Res<MouseCoordinates>,
) {
    let left_weapon = res_equipped_weapons.left.expect("no left weapon");
    let mut weapon = q_weapon
        .get_mut(left_weapon)
        .expect("weapon has no transform");
    let cooldown_finished = &mut weapon.1.0;
    if !*cooldown_finished {
        return;
    };
    *cooldown_finished = false;
    let weapon_tip_translation = q_tip_transform.into_inner().translation();
    let weapon_vec2 = Vec2 {
        x: weapon_tip_translation.x,
        y: weapon_tip_translation.y,
    };
    let mouse_coords = mouse_coords.0 - weapon_vec2;
    if let Some(projectile) = weapon.2 {
        commands
            .spawn((
                ProjectileBuilder::build(
                    projectile.clone(),
                    Dir2::try_from(mouse_coords).expect("invalid mouse coords"),
                ),
                Transform::from_translation(weapon_tip_translation),
            ))
            .observe(get_hits);
    }
}

pub fn aim_weapon(
    mut transform: Single<&mut Transform, With<RotationCenter>>,
    window: Single<&Window>,
) {
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let cursor_pos_frac = cursor_pos / window.size();
    let cursor_pos_signed = cursor_pos_frac - Vec2::splat(0.5);

    let angle = cursor_pos_signed.y.atan2(cursor_pos_signed.x);
    transform.rotation = Quat::from_rotation_z(-angle);
}
