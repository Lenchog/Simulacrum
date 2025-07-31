use crate::{prelude::*, weapons::prelude::*};

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

#[derive(Event)]
pub struct Unhook;

pub fn handle_grapple_hook(
    q_projectile: Query<(Entity, &Transform, &ProjectileType)>,
    q_player: Single<&GlobalTransform, With<Player>>,
    mut ev_unhook: EventWriter<Unhook>,
    mut commands: Commands,
) {
    for (entity, transform, projectile_type) in q_projectile {
        if *projectile_type == ProjectileType::Hook {
            let distance = transform.translation.distance(q_player.translation());
            match distance {
                1000.0.. => {
                    commands.entity(entity).insert(Retracting);
                }
                ..100.0 => {
                    ev_unhook.write(Unhook);
                }
                _ => {}
            };
        }
    }
}

pub fn unhook(
    mut ev_unhook: EventReader<Unhook>,
    mut q_velocity: Query<&mut LinearVelocity>,
    q_projectile: Query<(Entity, &ProjectileType)>,
    q_hooked: Query<Entity, With<Hooked>>,
    mut commands: Commands,
) {
    for _ in ev_unhook.read() {
        for (entity, projectile_type) in q_projectile {
            if *projectile_type == ProjectileType::Hook {
                commands.entity(entity).despawn();
                let mut velocity = q_velocity
                    .get_mut(entity)
                    .expect("Hook doesn't have velocity");
                velocity.0 = Vec2::ZERO;
                if let Ok(entity) = q_hooked.single() {
                    commands.entity(entity).remove::<Hooked>();
                }
            }
        }
    }
}

pub fn retract_hook(
    q_player: Single<(Entity, &GlobalTransform, &mut LinearVelocity), With<Player>>,
    q_hook: Single<(&mut LinearVelocity, &Transform), (With<Retracting>, Without<Player>)>,
    q_hooked: Query<
        Option<(&Hookable, &mut LinearVelocity)>,
        (With<Hooked>, Without<Retracting>, Without<Player>),
    >,
    mut commands: Commands,
) {
    let (mut hook_velocity, hook_transform) = q_hook.into_inner();
    let (player, player_transform, mut player_velocity) = q_player.into_inner();
    let direction = (player_transform.translation() - hook_transform.translation)
        .truncate()
        .normalize();
    let hook_speed = hook_velocity.0.length();
    for hooked in q_hooked {
        if let Some((_, mut velocity)) = hooked {
            *velocity = *hook_velocity;
        } else {
            // pull player
            commands.entity(player).insert(Grounded);
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
