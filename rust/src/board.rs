use godot::classes::INode;
use godot::classes::Node;
use godot::prelude::*;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::board_cell::BoardCell;
use crate::neutron::Neutron;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Board {
    base: Base<Node>,
    #[export]
    pub map_size: Vector2i,
    pub cells: HashMap<Vector2i, BoardCell>,
    pub neutrons: HashMap<usize, Neutron>,
    pub id: Uuid,
    pub reaction_queue_of_cell_positions: Vec<Vector2i>,
    pub unresolved_reaction_queue_of_cell_positions: Vec<Vector2i>,
}

#[godot_api]
pub(crate) impl INode for Board {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            map_size: Vector2i { x: 5, y: 5 },
            cells: HashMap::<Vector2i, BoardCell>::new(),
            neutrons: HashMap::<usize, Neutron>::new(),
            id: Uuid::new_v4(),
            reaction_queue_of_cell_positions: Vec::new(),
            unresolved_reaction_queue_of_cell_positions: Vec::new(),
        }
    }

    fn ready(&mut self) {
        // let ending_cell = Vector2i {
        //     x: self.map_size.x - 1,
        //     y: self.map_size.y - 1,
        // };

        self.cells = self.create_cells(self.map_size);
        // let wall_positions = HashSet::from([Vector2i { x: 2, y: 2 }]);
        // self.apply_walls(wall_positions, ending_cell);
        // self.print();
    }
}

#[godot_api]
pub(crate) impl Board {
    pub(crate) fn get_neighboring_cells_positions(
        &self,
        position: &Vector2i,
        starting_cell: i32,
        ending_cell: Vector2i,
    ) -> Vec<Vector2i> {
        // This is where the clockwise dispersion is defined.
        let directions = [
            Vector2i { x: 0, y: -1 },
            Vector2i { x: 1, y: 0 },
            Vector2i { x: 0, y: 1 },
            Vector2i { x: -1, y: 0 },
        ];

        directions
            .into_iter()
            .map(|direction| position.to_owned() + direction)
            .filter(|neighbor| {
                neighbor.x >= starting_cell
                    && neighbor.y >= starting_cell
                    && neighbor.x <= ending_cell.x
                    && neighbor.y <= ending_cell.y
            })
            .collect()
    }

    pub(crate) fn get_neighboring_cells(
        &self,
        position: &Vector2i,
        starting_cell: i32,
        ending_cell: Vector2i,
    ) -> Vec<&BoardCell> {
        self.get_neighboring_cells_positions(position, starting_cell, ending_cell)
            .iter()
            .filter_map(|neighbor_position| self.cells.get(neighbor_position))
            .collect()
    }

    fn calculate_max_neutron(&self, position: Vector2i, ending_cell: Vector2i) -> usize {
        let mut wall_count = 0;

        if position.x == 0 || position.x == ending_cell.x {
            wall_count += 1;
        }

        if position.y == 0 || position.y == ending_cell.y {
            wall_count += 1;
        }

        4 - wall_count
    }

    fn create_cells(&self, map_size: Vector2i) -> HashMap<Vector2i, BoardCell> {
        let ending_cell = Vector2i {
            x: map_size.x - 1,
            y: map_size.y - 1,
        };
        let mut cells = HashMap::new();

        for y in 0..map_size.y {
            for x in 0..map_size.x {
                let position = Vector2i { x, y };

                cells.insert(
                    position,
                    BoardCell {
                        position,
                        max_neutrons: self.calculate_max_neutron(position, ending_cell),
                        neutron_ids: Vec::new(),
                        unresolved_neutrons_ids: Vec::new(),
                    },
                );
            }
        }

        cells
    }

    fn apply_walls(&mut self, wall_positions: HashSet<Vector2i>, ending_cell: Vector2i) {
        for wall_position in wall_positions {
            if let Some(cell) = self.cells.get_mut(&wall_position) {
                cell.max_neutrons = 0;
            }

            let neighbor_positions =
                self.get_neighboring_cells_positions(&wall_position, 0, ending_cell);

            for neighbor_position in neighbor_positions {
                if let Some(cell) = self.cells.get_mut(&neighbor_position) {
                    cell.max_neutrons = cell.max_neutrons.saturating_sub(1);
                }
            }
        }
    }

    pub(crate) fn print(&self, print_details: bool) {
        godot_print!("Printing Board:");

        for y in 0..self.map_size.y {
            let mut row = String::new();

            for x in 0..self.map_size.x {
                let position = Vector2i { x, y };

                match self.cells.get(&position) {
                    Some(cell) if print_details => {
                        if cell.neutron_ids.is_empty() {
                            row.push_str(". ");
                        } else {
                            row.push_str(&format!("{} ", cell.neutron_ids.len(),));
                        }
                    }
                    Some(cell) => row.push_str(&format!("{} ", cell.max_neutrons)),
                    None => row.push_str(". "),
                }
            }

            godot_print!("{}", row);
        }

        godot_print!("Total cells: {}", self.cells.len());
    }
}
