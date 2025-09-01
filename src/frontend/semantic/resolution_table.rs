use std::collections::HashMap;

use crate::frontend::{hir::node_id::NodeId, syntax::ty::Ty};

#[derive(Default, Debug)]
pub struct ResolutionTable {
    variable_offsets: HashMap<NodeId, usize>,
    resolutions: HashMap<NodeId, Resolution>,
    type_resolutions: HashMap<NodeId, Ty>,
}

impl ResolutionTable {
    pub fn insert_variable_offset(&mut self, id: NodeId, offset: usize) {
        self.variable_offsets.insert(id, offset);
    }

    pub fn create_resolution(&mut self, id: NodeId, target: NodeId) {
        let resolution = Resolution::Node(target);

        self.name_resolutions.insert(id, resolution);
    }

    pub fn get_name_resolution(&self, id: &NodeId) -> Option<&Resolution> {
        self.name_resolutions.get(id)
    }

    pub fn create_type_resolution(&mut self, id: NodeId, ty: Ty) {
        self.type_resolutions.insert(id, ty);
    }

    pub fn get_type_resolution(&self, id: &NodeId) -> Option<&Ty> {
        self.type_resolutions.get(id)
    }
}
