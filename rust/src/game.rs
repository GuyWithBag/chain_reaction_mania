use godot::classes::INode;
use godot::classes::Node;
use godot::prelude::*;

use crate::board::Board;
use crate::board_actions::BoardActions;

#[derive(GodotClass)]
#[class(base=Node)]
pub(crate) struct Game {
    base: Base<Node>,
}

#[godot_api]
pub(crate) impl INode for Game {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
    fn ready(&mut self) {
        let mut board_node = Board::new_alloc();
        // let scene_tree = board_node.get_tree();
        // if let Some(scene) = scene_tree.get_current_scene() {
        //     scene.add_child(board_node);
        // }
        self.base_mut().add_child(&board_node);

        let mut board = board_node.bind_mut();

        board.print(true);
        BoardActions::add_neutron(&mut board, Vector2i { x: 2, y: 2 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 3, y: 2 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 3, y: 3 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 1, y: 2 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 1, y: 3 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 1, y: 1 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 2, y: 1 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 3, y: 1 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 2, y: 3 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 2, y: 2 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 3, y: 2 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 3, y: 3 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 1, y: 2 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 1, y: 3 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 1, y: 1 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 2, y: 1 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 3, y: 1 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 2, y: 3 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 2, y: 2 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 3, y: 2 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 3, y: 3 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 1, y: 2 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 1, y: 3 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 1, y: 1 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 2, y: 1 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 3, y: 1 });
        BoardActions::add_neutron(&mut board, Vector2i { x: 2, y: 3 });
        board.print(true);

        BoardActions::add_neutron(&mut board, Vector2i { x: 2, y: 2 });
        board.print(true);
    }
}
