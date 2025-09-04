use crate::prelude::*;

pub struct CollectablePlugin;

impl Plugin for CollectablePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Unlocks {
            max_energy: 100,
            max_health: 200,
            dash: false,
            double_jump: false,
            grapple_hook: false,
        })
        .add_event::<CollectableEvent>()
        .add_systems(FixedUpdate, collectable);
    }
}

#[derive(Component, Default, Clone)]
#[require(
    Collider::rectangle(64.0, 128.0),
    Sensor,
    CollisionLayers::new(PhysicsLayers::Collectable, PhysicsLayers::Player),
    CollisionEventsEnabled
)]
pub enum CollectableType {
    #[default]
    Battery,
    Dash,
    DoubleJump,
}

#[derive(Resource)]
pub struct Unlocks {
    pub max_energy: u32,
    pub max_health: u32,
    pub dash: bool,
    pub double_jump: bool,
    pub grapple_hook: bool,
}

pub fn collectable(
    q_energy: Single<&mut Energy, With<Player>>,
    mut ev_collectable: EventReader<CollectableEvent>,
    mut r_unlocks: ResMut<Unlocks>,
    mut commands: Commands,
) {
    let mut energy = q_energy.into_inner();
    for event in ev_collectable.read() {
        match event.1 {
            CollectableType::Battery => {
                let added_energy = 25;
                if energy.0 + added_energy > r_unlocks.max_energy {
                    energy.0 = r_unlocks.max_energy;
                } else {
                    energy.0 += added_energy;
                }
            }
            CollectableType::Dash => {
                r_unlocks.dash = true;
            }
            CollectableType::DoubleJump => {
                r_unlocks.double_jump = true;
            }
        }
        commands.entity(event.0).despawn();
    }
}

#[derive(Component, Default)]
#[require(CollectableType::Battery)]
pub struct Battery;

#[derive(Bundle, Default, LdtkEntity)]
pub struct BatteryBundle {
    battery: Battery,
    #[sprite_sheet]
    sprite: Sprite,
}

#[derive(Component, Default)]
#[require(CollectableType::Dash)]
pub struct DashUnlock;

#[derive(Bundle, Default, LdtkEntity)]
pub struct DashBundle {
    dash_unlock: DashUnlock,
    #[sprite]
    sprite: Sprite,
}
