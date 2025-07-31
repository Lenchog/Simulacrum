use crate::prelude::*;

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
