use bevy::{
    color::palettes::css::*, 
    prelude::*
};
use crate::maze;

pub struct Utils {}
pub struct RectDrawer<'a> {
    commands: Commands<'a, 'a>, 
    meshes: ResMut<'a, Assets<Mesh>>,
    materials: ResMut<'a,Assets<ColorMaterial>>,
}
impl<'a> RectDrawer<'a> {
    pub fn new (
        commands: Commands<'a, 'a>, 
        meshes: ResMut<'a, Assets<Mesh>>,
        materials: ResMut<'a, Assets<ColorMaterial>>,
    ) -> RectDrawer<'a>  {

        RectDrawer {
            commands, 
            meshes,
            materials,
        }
    }
    pub fn spawn(&mut self, x: f32,y: f32, width: f32, height: f32, color: Color, z: f32)
    {
        self.commands.spawn((
            Mesh2d(self.meshes.add(Rectangle::new(width, height))),
            MeshMaterial2d(self.materials.add(color)), // RGB values exceed 1 to achieve a bright color for the bloom effect
            Transform::from_xyz(x, y, z),
        ));
    }
}

impl Utils {
    pub fn draw_maze (
        commands: Commands, 
        window: Query<&Window>,  
        width: usize, 
        maze_cells: Vec<maze::Cell>,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window.single();
    
    let cell_width = window.width() / width as f32;
    let cell_height = window.height() / width as f32;

    let left_cell_x = (window.width() / 2.0 - cell_width as f32 / 2.0) * -1.0;
    let top_cell_y = window.height() / 2.0 - cell_height as f32 / 2.0;

    let mut rect = RectDrawer::new(
        commands, 
        meshes,
        materials,
    );

    for (i, cell) in maze_cells.into_iter().enumerate() {
        let top = top_cell_y + ((i - i % width) / width) as f32 * cell_height * -1.0;
        let mut left =  left_cell_x + i as f32 % width as f32 * cell_width;
        if i > 0 && i % width == 0 {
            left =  left_cell_x;
        }

        rect.spawn(
            left,
            top,
            cell_width,
            cell_height,
            Color::Srgba(YELLOW_GREEN),
            2.
        );

        if cell.has_border_right {
            rect.spawn(
                left + cell_width / 2.0,
                top,
                2.0,
                cell_height,
                Color::Srgba(MIDNIGHT_BLUE),
                3.
            );
        }

        if cell.has_border_bottom {
            rect.spawn(
                left,
                top + cell_height / -2.0,
                cell_width,
                2.0,
                Color::Srgba(MIDNIGHT_BLUE),
                3.
            );
        }
    }
  }
}
