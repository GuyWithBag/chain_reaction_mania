use uuid::Uuid;

enum EventType {
    Reaction,
}

struct BoardEvent {
    id: Uuid,
    event_type: EventType,
}
