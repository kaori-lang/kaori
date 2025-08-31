use std::collections::HashMap;

use crate::frontend::hir::node_id::NodeId;

pub struct Table {
    offsets: HashMap<NodeId, usize>,
    resolutions: HashMap<NodeId, NodeId>,
}

impl Table {
    pub fn create_offset(&mut self, id: NodeId, offset: usize) {
        self.offsets.insert(id, offset);
    }

    pub fn create_resolution(&mut self, id: NodeId, target: NodeId) {
        self.resolutions.insert(id, target);
    }

    pub fn get_offset(&self, id: NodeId) -> Option<&usize> {
        self.offsets.get(&id)
    }

    pub fn get_resolution(&self, id: NodeId) -> Option<&NodeId> {
        self.resolutions.get(&id)
    }
}
