use crate::{prelude::*, weapons::prelude::*};

pub fn lazer_gun(asset_server: &AssetServer, tip_entity: Entity) -> impl Bundle {
    RangedWeaponBuilder {
        sprite: Sprite::from_image(asset_server.load("placeholder_gun.png")),
        projectile_builder: ProjectileBuilder {
            sprite: Sprite::from_image(asset_server.load("placeholder_bullet.png")),
            speed: 5000.0,
            damage: Damage(10),
            ..default()
        },
        ..default()
    }
    .build(tip_entity, WeaponType::Gun)
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
    .build(tip_entity, WeaponType::FastGun)
}

pub fn power_gun(asset_server: &AssetServer, tip_entity: Entity) -> impl Bundle {
    RangedWeaponBuilder {
        sprite: Sprite::from_image(asset_server.load("placeholder_gun.png")),
        usetime: UseTime(Timer::new(Duration::from_millis(600), TimerMode::Once)),
        projectile_builder: ProjectileBuilder {
            sprite: Sprite::from_image(asset_server.load("placeholder_bullet.png")),
            speed: 3000.0,
            damage: Damage(15),
            ..default()
        },
    }
    .build(tip_entity, WeaponType::PowerGun)
}
