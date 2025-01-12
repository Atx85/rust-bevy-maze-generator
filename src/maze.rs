use rand::prelude::*;
use bevy::prelude::Resource;

#[derive(Clone, Debug, Resource)]
pub struct Maze {
  pub width: usize,
  visited: Vec<usize>,
  maze: Vec<Cell>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
  pub has_border_right: bool,
  pub has_border_bottom: bool,
  pub neighbours: Vec<usize>
}

impl Maze {
  pub fn new(width: usize) -> Maze {
    let maze_vec = vec![Cell {
      has_border_bottom: true,
      has_border_right: true,
      neighbours: Vec::new()
    }; width * width];
    Maze {
      width,
      visited: Vec::new(),
      maze: maze_vec,
    } 
  }

  pub fn get_cells (&mut self) -> Vec<Cell> {
    self.maze.clone()
  }

  pub fn update_cell_borders (&mut self, last_cell_id: usize, actual_cell_id: usize) {
    if last_cell_id as i32 - actual_cell_id as i32 == -(self.width as i32) {
      self.maze[last_cell_id].has_border_bottom = false;
    }

    if last_cell_id as i32 - actual_cell_id as i32 == (self.width as i32) {
      self.maze[actual_cell_id].has_border_bottom = false;
    }

    if last_cell_id as i32 - actual_cell_id as i32 == -1 {
      self.maze[last_cell_id].has_border_right = false
    }

    if last_cell_id as i32 - actual_cell_id as i32 == 1 {
      self.maze[actual_cell_id].has_border_right = false
    }
  }

  pub fn generate (&mut self) -> Vec<Cell> {
    let mut stack: Vec<usize> = Vec::new();

    stack.push(0);
    self.visited.push(0);

    while self.visited.len() < (self.width * self.width) {
      if let Some(last_cell_id) = stack.last_mut() {
          let last_cell_id = *last_cell_id;
          if let Ok(next_cell) = self.get_next_cell_id(&last_cell_id) {

            // here we have a next cell
            stack.push(next_cell);
            self.visited.push(next_cell);
            self.update_cell_borders(last_cell_id, next_cell);
            
            // set actual which might not be needed anymore
          } else {
            // we don't have a next cell and we need to backpropagate 
            stack.pop();
          
          }
      }
    }
    self.maze.clone()
  }

  fn can_have_neighbour(&self, actual: usize, next: i32, width: usize) -> bool {
      if next < 0 {
        return false;
      } 
      let grid_size = (width * width) as i32;
      let can_have_right = (actual % width) + 1 < (width);
      let can_have_left = (actual % width) > 0;
      let valid_position = (next as i32) < grid_size;
      
      match (next as i32) - (actual as i32) {
          -1 => valid_position && can_have_left,
          1 => valid_position && can_have_right,
          _ => valid_position
      }
  }

  fn get_next_cell_id(&mut self, index: &usize ) -> core::result::Result<usize, &'static str> {
    let mut possible_neighbours = Vec::new();
    let mut rng = rand::thread_rng();

    for modifier in [1, ((self.width) as i32) * -1, -1, (self.width) as i32].into_iter() { 
      let next_possible_neighbour_index = (*index as i32) + modifier;
     
       if self.can_have_neighbour(*index, next_possible_neighbour_index.clone() , self.width)
        && !self.visited.contains(&(next_possible_neighbour_index as usize))
       {
        possible_neighbours.push((*index as i32 + modifier) as usize);
        self.maze[*index].neighbours.push((*index as i32 + modifier) as usize);
       }
    }

    let next_neighbour_id;

    // let num = rng.gen_range(1 .. 2);
    if possible_neighbours.len() > 0 {
      if possible_neighbours.len() == 1 {
        next_neighbour_id = possible_neighbours[0];
      } else {
        let next_neigbour_id_index = rng.gen_range(0 .. possible_neighbours.len());
        next_neighbour_id = possible_neighbours[next_neigbour_id_index];
      } 
      return Ok(next_neighbour_id);
    } else {
      return Err("Could not find a neighbour");
    }
  }
}


#[cfg(test)]
mod tests {
    use super::Maze;
    use super::Cell;
    const WIDTH: usize = 3;

    fn type_of<T>(_: &T) -> String  {
      format!("{}", std::any::type_name::<T>())
    }

    macro_rules! assert_is_type {
      ($t:ty, $i:ident: $ti:ty) => {
          const _: () = {
              fn dummy(v: $t) {
                  let _: $ti = v.$i;
              }
          };
      }
  }

    #[test]
    fn cell() {
      assert_is_type!(Cell, has_border_bottom: bool);
      assert_is_type!(Cell, has_border_right: bool);
    }

    #[test]
    fn init() {
      let m = Maze::new(WIDTH);
      assert_eq!(m.width, 3);
      assert_eq!(m.visited.len(), 0);
      assert_eq!(m.maze.len(), WIDTH * WIDTH);
    }

    #[test]
    fn get_cells() {
      let mut m = Maze::new(WIDTH);
      assert_eq!("alloc::vec::Vec<maze_gen::maze::Cell>" ,type_of(&m.get_cells()));
    }

    // Making sure that the right border gets enabled
    // last cell id cell is expected to be modified on moving right and up, otherwise the actual cell id
    #[test]
    fn update_cell_borders() {
      // moving right
      let mut m = Maze::new(WIDTH);
      m.update_cell_borders(1, 2);
      assert_eq!(false, m.maze[1].has_border_right);
      assert_eq!(true, m.maze[1].has_border_bottom);
      assert_eq!(true, m.maze[2].has_border_right);
      assert_eq!(true, m.maze[2].has_border_bottom);
      
      // moving left
      let mut m = Maze::new(WIDTH);
      m.update_cell_borders(1, 0);
      assert_eq!(false, m.maze[0].has_border_right);
      assert_eq!(true, m.maze[0].has_border_bottom);
      assert_eq!(true, m.maze[1].has_border_right);
      assert_eq!(true, m.maze[1].has_border_bottom);

            
      // moving down
      let mut m = Maze::new(WIDTH);
      m.update_cell_borders(1, 4);
      assert_eq!(false, m.maze[1].has_border_bottom);
      assert_eq!(true, m.maze[1].has_border_right);
      assert_eq!(true, m.maze[4].has_border_bottom);
      assert_eq!(true, m.maze[4].has_border_right);
            
      // moving up
      let mut m = Maze::new(WIDTH);
      m.update_cell_borders(7, 4);
      assert_eq!(false, m.maze[4].has_border_bottom);
      assert_eq!(true, m.maze[4].has_border_right);
      assert_eq!(true, m.maze[7].has_border_bottom);
      assert_eq!(true, m.maze[7].has_border_right);
    }

    #[test]
    fn get_next_cell_id() {
        let mut m = Maze::new(WIDTH);
        // 0 1 2
        // 3 4 5
        // 6 7 8
        // checking for all the fields if they have a valid possible neighbour
        for last_element_id in 0..9 {
          let mut possible_neighbours = Vec::new();
          for modifier in [1, ((m.width) as i32) * -1, -1, (m.width) as i32].into_iter() { 
            let next_possible_neighbour_index = (last_element_id as i32) + modifier;
           
             if m.can_have_neighbour(last_element_id, next_possible_neighbour_index.clone() , m.width)
              && !m.visited.contains(&(next_possible_neighbour_index as usize))
             {
              possible_neighbours.push((last_element_id as i32 + modifier) as usize);
             }
          }
          let res = m.get_next_cell_id(&last_element_id).unwrap();
          assert_eq!(possible_neighbours.contains(&res) ,true);
        }         
    }

    #[test]
    fn can_have_next() {
      let m = Maze::new(WIDTH);
      assert_eq!(m.can_have_neighbour(5, 6, WIDTH), false);
      assert_eq!(m.can_have_neighbour(5, 4, WIDTH), true);
      assert_eq!(m.can_have_neighbour(5, 2, WIDTH), true);
      assert_eq!(m.can_have_neighbour(5, 8, WIDTH), true);
      assert_eq!(m.can_have_neighbour(3, 2, WIDTH), false);
      assert_eq!(m.can_have_neighbour(7, 8, WIDTH), true);
    }
}