use crate::{
    mouse::MouseCoordinates,
    robot::player::{Energy, Player, input::Attack, weapons::*},
};
use avian2d::math::PI;
use bevy_simple_subsecond_system::hot;

pub fn weapon_cooldown(
    q_weapon: Query<(Entity, &mut UseTime, &mut CooldownFinished)>,
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
            &mut CooldownFinished,
            Option<&ProjectileBuilder>,
            Option<&Swingable>,
        ),
        With<Weapon>,
    >,
    q_hook: Query<(Entity, &ProjectileType)>,
    q_hooked: Query<Entity, With<Hooked>>,
    q_weapon_entity: Query<Entity, With<Weapon>>,
    q_tip_transform: Single<&GlobalTransform, With<WeaponTip>>,
    q_rotation_center: Single<Entity, (Without<SwingRotation>, With<RotationCenter>)>,
    q_player: Single<&mut Energy, With<Player>>,
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
    for weapon in q_weapon_entity {
        commands.entity(weapon).remove::<Equipped>();
    }
    commands.entity(weapon_entity).insert(Equipped);
    let (mut cooldown, projectile_builder, swingable) = q_weapon
        .get_mut(weapon_entity)
        .expect("could not get active weapon");
    if !cooldown.0 {
        return;
    };
    cooldown.0 = false;
    let weapon_tip_translation = q_tip_transform.into_inner().translation();
    let weapon_vec2 = Vec2 {
        x: weapon_tip_translation.x,
        y: weapon_tip_translation.y,
    };
    let mouse_coords = mouse_coords.0 - weapon_vec2;

    if let Some(projectile) = projectile_builder {
        for (entity, projectile_type) in q_hook {
            // if the grappling hook is already out, reset instead of shooting it
            if *projectile_type == ProjectileType::Hook
                && projectile.projectile_type == ProjectileType::Hook
            {
                commands.entity(entity).despawn();
                for hooked in q_hooked {
                    commands.entity(hooked).remove::<Hooked>();
                }
                return;
            }
        }
        let mut energy = q_player.into_inner();
        if energy.0 == 0 {
            return;
        }
        energy.0 = energy.0.saturating_sub(projectile.energy_cost.0);
        commands.spawn((
            ProjectileBuilder::build(
                projectile.clone(),
                Dir2::try_from(mouse_coords).expect("invalid mouse coords"),
            ),
            Transform::from_translation(weapon_tip_translation),
        ));
    } else if swingable.is_some() {
        commands.entity(weapon_entity).remove::<ColliderDisabled>();
        commands
            .entity(q_rotation_center.into_inner())
            .insert(SwingRotation(0.0));
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
