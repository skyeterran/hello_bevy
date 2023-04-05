use bevy::prelude::*;

// Physics timestep
const TIMESTEP: f32 = 1.0 / 60.0;
const CRITTER_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hello_setup)
           .add_system(move_critter)
           .add_system(update_tracker)
           .insert_resource(FixedTime::new_from_secs(TIMESTEP))
           .insert_resource(Tracker { distance: 0.0 });
    }
}

fn hello_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(CritterBundle::new(&asset_server));

    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
        ..default()
    };

    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Distance traveled: ",
                text_style.clone(),
            ),
            TextSection::from_style(text_style.clone()),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            ..default()
        }),
    );
}

#[derive(Resource)]
struct Tracker {
    distance: f32,
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Critter;

#[derive(Bundle)]
struct CritterBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    critter: Critter,
}

impl CritterBundle {
    fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("images/Streetlet.png"),
                transform: Transform {
                    scale: Vec3::splat(0.25),
                    ..default()
                },
                ..default()
            },
            collider: Collider,
            critter: Critter,
        }
    }
}

fn move_critter(
    keyboard_input: Res<Input<KeyCode>>,
    mut tracker: ResMut<Tracker>,
    mut query: Query<&mut Transform, With<Critter>>,
) {
    let mut critter_transform = query.single_mut();
    let mut direction: Vec2 = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }

    if direction.length_squared() > 1.0 {
        direction = direction.normalize();
    }

    let delta = direction * CRITTER_SPEED * TIMESTEP;

    let new_critter_pos = critter_transform.translation + Vec3::new(delta.x, delta.y, 0.0);

    critter_transform.translation = new_critter_pos;

    tracker.distance += delta.length();
}

fn update_tracker(
    tracker: Res<Tracker>,
    mut query: Query<&mut Text>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = ((tracker.distance / 100.0) as usize).to_string();
}
