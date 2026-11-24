use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Component, Res, ResMut, Resource};
use bevy::time::{Time, Timer, TimerMode};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hello_world)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn hello_world(time: Res<Time>, mut timer: ResMut<GreetTimer>){
    if timer.0.tick(time.delta()).just_finished() {
        println!("hello world!");
    }
}