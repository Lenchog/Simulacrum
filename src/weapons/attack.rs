use crate::{prelude::*, weapons::prelude::*};
use bevy_enhanced_input::prelude::Fired;

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
    q_weapon_entity: Query<Entity, With<Weapon>>,
    q_rotation_center: Single<Entity, (Without<SwingRotation>, With<RotationCenter>)>,
    res_equipped_weapons: Res<EquippedWeapons>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut ev_shoot: EventWriter<ShootEvent>,
    mut commands: Commands,
) {
    let Some(weapon_entity) = (if buttons.pressed(MouseButton::Right) {
        res_equipped_weapons.right
    } else {
        res_equipped_weapons.left
    }) else {
        return;
    };
    let (mut cooldown, projectile_builder, swingable) = q_weapon
        .get_mut(weapon_entity)
        .expect("could not get active weapon");
    commands.entity(weapon_entity).insert(Visibility::Visible);
    for weapon in q_weapon_entity {
        commands.entity(weapon).remove::<Equipped>();
    }
    commands.entity(weapon_entity).insert(Equipped);
    if !cooldown.0 {
        return;
    };
    cooldown.0 = false;

    if projectile_builder.is_some() {
        ev_shoot.write(ShootEvent(weapon_entity));
    } else if swingable.is_some() {
        commands.entity(weapon_entity).remove::<ColliderDisabled>();
        commands
            .entity(q_rotation_center.into_inner())
            .insert(SwingRotation(0.0));
    }
}
