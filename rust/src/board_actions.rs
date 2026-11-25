use std::mem::take;

use crate::{
    board::Board,
    board_events::{EventType, GameManager},
    neutron::Neutron,
};
use godot::prelude::*;

pub(crate) struct BoardActions;

impl BoardActions {
    pub(crate) fn add_neutron(board: &mut Board, position: Vector2i) {
        if !board.cells.contains_key(&position) {
            return;
        }

        let neutron_id = board.neutrons.len();
        board.neutrons.insert(
            neutron_id,
            Neutron {
                position,
                id: neutron_id,
            },
        );

        let should_disperse = {
            let cell = board
                .cells
                .get_mut(&position)
                .expect("cell was checked before inserting the neutron");

            cell.neutron_ids.push(neutron_id);
            cell.neutron_ids.len() >= cell.max_neutrons
        };

        if should_disperse {
            // Self::disperse(board, &position);
            board.reaction_queue_of_cell_positions.push(position);
            GameManager::singleton()
                .bind_mut()
                .log(EventType::AddNeutron {
                    id: neutron_id,
                    position,
                });
            Self::chain_react(board);
        }
    }

    fn disperse(board: &mut Board, initial_position: &Vector2i) {
        let ending_cell = Vector2i {
            x: board.map_size.x - 1,
            y: board.map_size.y - 1,
        };

        let neighbor_positions =
            board.get_neighboring_cells_positions(initial_position, 0, ending_cell);

        if let Some(initial_cell) = board.cells.get_mut(initial_position) {
            if initial_cell.neutron_ids.len() != initial_cell.max_neutrons {
                // board
                //     .reaction_queue_of_cell_positions
                //     .retain(|item| item != initial_position);
                return;
            }
        }

        for neighbor_position in neighbor_positions {
            // Moving the neutrons to the neighbores: By removing the initial neutrons.
            let initial_neutron_id = match board.cells.get_mut(initial_position) {
                Some(cell) => match cell.neutron_ids.pop() {
                    Some(id) => id,
                    None => continue,
                },
                None => return,
            };

            if let Some(neighbor_cell) = board.cells.get_mut(&neighbor_position) {
                // If cell is already >= cell.max_neutrons then add to unresolved.
                if neighbor_cell.neutron_ids.len() >= neighbor_cell.max_neutrons {
                    neighbor_cell
                        .unresolved_neutrons_ids
                        .push(initial_neutron_id);
                    board
                        .unresolved_reaction_queue_of_cell_positions
                        .push(neighbor_position);

                    // if that neighbor cell which now has unresolved neutrons, has neutrons from unresolved neutrons, then remove it.
                    // neighbor_cell
                    //     .neutron_ids
                    //     .retain(|neutron_id| *neutron_id != initial_neutron_id);
                } else {
                    neighbor_cell.neutron_ids.push(initial_neutron_id);
                    // If the cell now has 4 neutrons after pushing, then add to queue.
                    let neighbor_cell_neutrons = neighbor_cell.neutron_ids.len();
                    let neighbor_cell_max_neutrons = neighbor_cell.max_neutrons;
                    // board.print_reaction_queue();
                    if neighbor_cell_neutrons == neighbor_cell_max_neutrons {
                        godot_print!(
                            "Pushed '{neighbor_position}' because neighbor cell '{neighbor_position}' is {neighbor_cell_neutrons} >= {neighbor_cell_max_neutrons} "
                        );
                        board
                            .reaction_queue_of_cell_positions
                            .push(neighbor_position);
                    }
                    GameManager::singleton()
                        .bind_mut()
                        .log(EventType::Reaction {
                            happened_to: neighbor_position,
                            happened_from: *initial_position,
                            neutron_id: initial_neutron_id,
                        });
                }
            }
            // Updating existing neutron only. (Should be)
            if let Some(neutron) = board.neutrons.get_mut(&initial_neutron_id) {
                neutron.position = neighbor_position;
            }
        }

        // After all that, now resolve unresolved neutrons by puttin it in the initial_cell.
        if let Some(initial_cell) = board.cells.get_mut(initial_position) {
            initial_cell
                .neutron_ids
                .append(&mut initial_cell.unresolved_neutrons_ids);
        }

        board.print(true);
        // board.print_reaction_queue();
        // board.print_unresolved_reaction_queue();
        // Chain react?
        // Self::chain_react(board);
    }

    fn disperse_queue(board: &mut Board) {
        board.print_reaction_queue();

        let reaction_queue = take(&mut board.reaction_queue_of_cell_positions);
        // let reaction_queue = &mut board.reaction_queue_of_cell_positions.to_owned();

        for cell_position in reaction_queue {
            // let cell = board.cells.get_mut(cell_position);
            Self::disperse(board, &cell_position);
        }

        // board.reaction_queue_of_cell_positions.clear();
    }

    fn disperse_unresolved_reactions_queue(board: &mut Board) {
        board.print_unresolved_reaction_queue();

        let unresolved_reaction_queue =
            take(&mut board.unresolved_reaction_queue_of_cell_positions);
        // let unresolved_reaction_queue =
        //     &mut board.unresolved_reaction_queue_of_cell_positions.to_owned();

        for cell_position in unresolved_reaction_queue {
            // let cell = board.cells.get_mut(cell_position);
            Self::disperse(board, &cell_position);
        }

        // board.unresolved_reaction_queue_of_cell_positions.clear();
    }

    fn chain_react(board: &mut Board) {
        Self::disperse_queue(board);
        Self::disperse_unresolved_reactions_queue(board);

        if board.reaction_queue_of_cell_positions.is_empty()
            && board.unresolved_reaction_queue_of_cell_positions.is_empty()
        {
            return;
        }

        Self::chain_react(board);
    }
}
