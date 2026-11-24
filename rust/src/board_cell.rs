use godot::prelude::*;

pub(crate) struct BoardCell {
    pub(crate) max_neutrons: usize,
    pub(crate) neutron_ids: Vec<usize>,
    pub(crate) unresolved_neutrons_ids: Vec<usize>,
    pub(crate) position: Vector2i,
}
