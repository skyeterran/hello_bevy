use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
           .add_system(hello_world)
           .add_system(greet_people);
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Skye".to_string())));
    commands.spawn((Person, Name("Salad".to_string())));
    commands.spawn((Person, Name("Soup".to_string())));
}

fn hello_world() {
    println!("Hello, world!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hi there, {}!", name.0);
    }
}
