
use bevy::prelude::*;
use crate::maze::Maze;

pub struct PathfinderPluigin;

impl Plugin for PathfinderPluigin {
  fn build(&self, app: &mut App) {
      app.add_system(pathfinding_system);
  }
}

fn pathfinding_system(commands: Commands, mut maze: ResMut<Maze>) {
  let cells = maze.get_cells();
  let mut q: Vec<usize> = Vec::new();
  let goal = maze.width * maze.width - 1;
  let mut visited: Vec<usize> = Vec::new();
  let mut stack: [usize; 25] = [0; 25];
  visited.push(0);
  q.push(0);
  stack[0] = 0;
  while q.len() > 0 {
    if let Some(v) = q.pop() {
      println!("{} {}", v, goal);
      if v == goal {
        // print!("found {:?}", v);
        break;
      }
      for edge in cells[v].neighbours.clone() {
        if !visited.contains(&edge) {
          visited.push(edge);
          q.push(edge);
          stack[edge] = v;
        }
      }
    }
  }
  println!("after");
  // println!("{:?}", stack);
  for i in stack {
    print!(" {}", stack[i]);
  }
}

#[derive(Debug)]
struct pathCell {
  value: usize,
  parent: usize
}

fn BFS() -> Vec<pathCell> {


  vec![pathCell{
    value: 1,
    parent: 1
  }]
}