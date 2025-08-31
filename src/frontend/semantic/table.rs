use std::collections::HashMap;

use crate::frontend::hir::node_id::NodeId;

#[derive(Clone)]
pub enum Resolution {
    Offset(usize),
    Node(NodeId),
}
pub struct Table {
    resolutions: HashMap<NodeId, Resolution>,
}

impl Table {
    pub fn create_local_resolution(&mut self, id: NodeId, offset: usize) {
        let resolution = Resolution::Offset(offset);

        self.resolutions.insert(id, resolution);
    }

    pub fn create_global_resolution(&mut self, id: NodeId, target: NodeId) {
        let resolution = Resolution::Node(target);

        self.resolutions.insert(id, resolution);
    }

    pub fn get_resolution(&self, id: &NodeId) -> Option<&Resolution> {
        self.resolutions.get(id)
    }
}
