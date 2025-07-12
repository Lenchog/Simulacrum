use crate::{
    MouseCoordinates,
    robot::{
        health::get_hits,
        player::{input::Attack, weapons::*},
    },
};
use bevy_enhanced_input::prelude::*;

pub fn weapon_cooldown(
    q_weapon: Query<(Entity, &mut UseTime, &mut CooldownFinished), With<Weapon>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut timer, mut cooldown_finished) in q_weapon {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).insert(Visibility::Hidden);
            commands.entity(entity).remove::<CollisionEventsEnabled>();
            *cooldown_finished = CooldownFinished(true);
            timer.0.reset();
        }
    }
}

pub fn attack(
    _: Trigger<Fired<Attack>>,
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
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    let Some(weapon_entity) = (if buttons.pressed(MouseButton::Right) {
        res_equipped_weapons.right
    } else {
        res_equipped_weapons.left
    }) else {
        return;
    };
    commands.entity(weapon_entity).insert(Visibility::Visible);
    commands
        .entity(weapon_entity)
        .insert(CollisionEventsEnabled);
    let mut weapon = q_weapon
        .get_mut(weapon_entity)
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
