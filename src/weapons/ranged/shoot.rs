use crate::prelude::*;

pub fn shoot(
    mut ev_shoot: EventReader<ShootEvent>,
    mut ev_unhook: EventWriter<Unhook>,
    q_tip_transform: Single<&GlobalTransform, With<WeaponTip>>,
    mouse_coords: Res<MouseCoordinates>,
    q_energy: Single<&mut Energy, With<Player>>,
    q_hook: Query<&ProjectileType>,
    mut q_weapon: Query<&ProjectileBuilder, With<Weapon>>,
    mut commands: Commands,
) {
    let mut energy = q_energy.into_inner();
    for event in ev_shoot.read() {
        let weapon_tip_translation = q_tip_transform.translation();
        let weapon_vec2 = Vec2 {
            x: weapon_tip_translation.x,
            y: weapon_tip_translation.y,
        };
        let mouse_coords = mouse_coords.0 - weapon_vec2;
        let projectile = q_weapon
            .get_mut(event.0)
            .expect("could not get active weapon");
        for projectile_type in q_hook {
            // if the grappling hook is already out, reset instead of shooting it
            if *projectile_type == ProjectileType::Hook
                && projectile.projectile_type == ProjectileType::Hook
            {
                ev_unhook.write(Unhook);
                return;
            }
        }
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
    }
}
