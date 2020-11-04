use crate::sand::Sand;
use std::sync::{Arc, RwLock, Mutex};
use std::ops::Deref;
use std::mem;

pub struct World {
    pub width: usize,
    pub height: usize,
    grid_size: usize,
    grid_max_x: usize,
    grid_max_y: usize,
    grids: Vec<Vec<Arc<RwLock<Vec<Arc<RwLock<Sand>>>>>>>,
    pub sands: Vec<Arc<RwLock<Sand>>>,
}

impl World {
    fn new_grid(grid_max_x: usize, grid_max_y: usize) -> Vec<Vec<Arc<RwLock<Vec<Arc<RwLock<Sand>>>>>>> {
        let mut grid = Vec::with_capacity(grid_max_x + 1);
        for _ in 0..=grid_max_x {
            let mut row = Vec::with_capacity(grid_max_y + 1);
            for _ in 0..=grid_max_y {
                row.push(Arc::new(RwLock::new(vec![])));
            }
            grid.push(row);
        }
        grid
    }

    pub fn new(width: usize, height: usize, grid_size: usize) -> Self {
        let grid_max_x = width / grid_size;
        let grid_max_y = height / grid_size;

        Self {
            width,
            height,
            grid_size,
            grid_max_x,
            grid_max_y,
            grids: Self::new_grid(grid_max_x, grid_max_y),
            sands: vec![],
        }
    }

    pub fn add_sand(&mut self, sand: Arc<RwLock<Sand>>) {
        {
            let sand = sand.read().unwrap();
            let grid_x = sand.x as usize / self.grid_size;
            let grid_y = sand.y as usize / self.grid_size;
            &mut self.grids[grid_x][grid_y].write().unwrap()
        }.push(sand.clone());
        self.sands.push(sand.clone())
    }

    fn try_minus_1(num: usize) -> usize {
        if num == 0 {
            num
        } else {
            num - 1
        }
    }

    pub fn get_nearby(&self, x: f32, y: f32) -> Vec<Arc<RwLock<Sand>>> {
        let grid_x = x as usize / self.grid_size;
        let grid_y = y as usize / self.grid_size;
        let mut result = vec![];
        for grid_x in Self::try_minus_1(grid_x)..=grid_x + 1 {
            if grid_x <= self.grid_max_x {
                for grid_y in Self::try_minus_1(grid_y)..=grid_y + 1 {
                    if grid_y <= self.grid_max_y {
                        self.grids[grid_x][grid_y].read().unwrap()
                            .iter().for_each(|sand| {
                            result.push(sand.clone());
                        });
                    }
                }
            }
        }

        result
    }

    pub fn recreate_grid(&mut self) {
        for grid_y in 0..=self.grid_max_y {
            for grid_x in 0..=self.grid_max_x {
                let mut grid = self.grids[grid_x][grid_y].write().unwrap();

                for i in (0..grid.len()).rev() {
                    let sand = grid[i].read().unwrap();
                    let sand_grid_x = sand.x as usize / self.grid_size;
                    let sand_grid_y = sand.y as usize / self.grid_size;
                    drop(sand);
                    if sand_grid_y != sand_grid_y || sand_grid_x != sand_grid_x {
                        self.grids[sand_grid_x][sand_grid_y]
                            .write().unwrap()
                            .push(grid.swap_remove(i));
                    }
                }
            }
        }
    }
}
