use godot::prelude::*;
use uuid::Uuid;

pub enum EventType {
    Reaction {
        happened_to: Vector2i,
        happened_from: Vector2i,
        neutron_id: usize,
    },
    AddNeutron {
        id: usize,
        position: Vector2i,
    },
}

struct BoardEvent {
    board_id: Uuid,
    event_type: EventType,
}

// struct BoardEventLog {
//     board_id: Uuid,
// }
//
//
#[derive(GodotClass)]
#[class(init, singleton)]
pub(crate) struct GameManager {
    pub events: Vec<EventType>,
    base: Base<Object>,
}

impl GameManager {
    pub fn log(&mut self, event: EventType) {
        self.events.push(event);
    }
}
