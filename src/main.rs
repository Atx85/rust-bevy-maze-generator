use bevy::prelude::*;

mod maze;
mod utils;
// mod pathfinder;

use crate::maze::Maze;
use crate::utils::Utils;
// use crate::pathfinder::PathfinderPluigin;

pub const CLEAR: Color = Color::srgb(0.1, 0.1, 0.1);
const WIDTH: usize = 50;

fn main() {

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Maze".into(),
                name: Some("maze.app".into()),
                resolution: (800., 800.).into(),
                ..default()
            }),
            ..default()
          }))
        .add_systems(Startup, ((setup_camera, include_maze, init_maze_sytem).chain()))
        .run();
}
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            ..default()
        }
    ));
}
fn include_maze(mut commands: Commands) {
    let maze = Maze::new(WIDTH);
    commands.insert_resource(maze);
}

fn init_maze_sytem(
    commands: Commands, 
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut maze: ResMut<Maze>, 
    window: Query<&Window>) {

    maze.generate();
    
    Utils::draw_maze(
        commands, 
        window, 
        WIDTH, 
        maze.get_cells(),
        meshes,
        materials,
    ); 
}

