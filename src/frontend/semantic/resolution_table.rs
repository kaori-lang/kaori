use std::collections::HashMap;

use crate::frontend::{hir::node_id::NodeId, syntax::ty::Ty};

#[derive(Clone, Debug)]
pub enum Resolution {
    Offset(usize),
    Node(NodeId),
}

#[derive(Default, Debug)]
pub struct ResolutionTable {
    name_resolutions: HashMap<NodeId, Resolution>,
    type_resolutions: HashMap<NodeId, Ty>,
}

impl ResolutionTable {
    pub fn create_local_resolution(&mut self, id: NodeId, offset: usize) {
        let resolution = Resolution::Offset(offset);

        self.name_resolutions.insert(id, resolution);
    }

    pub fn create_global_resolution(&mut self, id: NodeId, target: NodeId) {
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
