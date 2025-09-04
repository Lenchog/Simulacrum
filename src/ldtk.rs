use crate::{prelude::*, weapons::Hitbox};

pub struct MyLdtkPlugin;

impl Plugin for MyLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..default()
            })
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<BatteryBundle>("Battery")
            .register_ldtk_entity::<DashBundle>("DashUnlock")
            .register_ldtk_entity::<EnemyBundle>("Enemy")
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<SpikeBundle>(2)
            .register_ldtk_int_cell::<PlatformBundle>(3)
            .add_systems(FixedUpdate, level_selection_follow_player);
    }
}

pub fn level_selection_follow_player(
    q_player: Single<&GlobalTransform, With<Player>>,
    q_levels: Query<(&LevelIid, &GlobalTransform)>,
    q_ldtk_projects: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    for (level_iid, level_transform) in q_levels.iter() {
        let ldtk_project = ldtk_project_assets
            .get(*q_ldtk_projects)
            .expect("ldtk project should be loaded before player is spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(level_iid.get())
            .expect("level should exist in only project");

        let level_bounds = Rect {
            min: Vec2::new(
                level_transform.translation().x,
                level_transform.translation().y,
            ),
            max: Vec2::new(
                level_transform.translation().x + level.px_wid as f32,
                level_transform.translation().y + level.px_hei as f32,
            ),
        };
        if level_bounds.contains(q_player.translation().truncate()) {
            *level_selection = LevelSelection::Iid(level_iid.clone());
        }
    }
}

#[derive(Default, Component)]
pub struct Respawnable;

#[derive(Component, Default)]
#[require(
    CollisionLayers::new(
        PhysicsLayers::Ground,
        [
            PhysicsLayers::Enemy,
            PhysicsLayers::Player,
            PhysicsLayers::PlayerHitbox,
            PhysicsLayers::EnemyHitbox,
        ],
    ),
    RigidBody::Static,
    Collider::rectangle(128.0, 128.0),
)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct WallBundle {
    wall: Wall,
    respawnable: Respawnable,
}

#[derive(Component, Default)]
#[require(
    Wall,
    Hitbox,
    Sensor,
    CollisionLayers::new(
        PhysicsLayers::Spikes,
        [PhysicsLayers::Enemy, PhysicsLayers::Player]
    )
)]
pub struct Spike;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct SpikeBundle {
    spike: Spike,
}

#[derive(Component, Default)]
#[require(
    Wall,
    TnuaGhostPlatform,
    CollisionLayers::new(PhysicsLayers::Ground, PhysicsLayers::Default)
)]
pub struct Platform;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct PlatformBundle {
    platform: Platform,
}
