use std::collections::HashMap;

use crate::frontend::syntax::node_id::NodeId;

use super::checked_ty::CheckedTy;

pub enum Resolution {
    Variable(NodeId),
    Struct(NodeId),
    Function(NodeId),
    Type(NodeId),
}

impl Resolution {
    pub fn variable(id: NodeId) -> Resolution {
        Resolution::Variable(id)
    }

    pub fn struct_(id: NodeId) -> Resolution {
        Resolution::Struct(id)
    }

    pub fn function(id: NodeId) -> Resolution {
        Resolution::Function(id)
    }
}

#[derive(Default)]
pub struct ResolutionTable {
    variable_offsets: HashMap<NodeId, usize>,
    name_resolutions: HashMap<NodeId, Resolution>,
    type_resolutions: HashMap<NodeId, CheckedTy>,
}

impl ResolutionTable {
    pub fn insert_variable_offset(&mut self, id: NodeId, offset: usize) {
        self.variable_offsets.insert(id, offset);
    }

    pub fn insert_name_resolution(&mut self, id: NodeId, resolution: Resolution) {
        self.name_resolutions.insert(id, resolution);
    }

    pub fn insert_type_resolution(&mut self, id: NodeId, ty: CheckedTy) {
        self.type_resolutions.insert(id, ty);
    }

    pub fn get_variable_offset(&self, id: &NodeId) -> Option<&usize> {
        self.variable_offsets.get(id)
    }

    pub fn get_name_resolution(&self, id: &NodeId) -> Option<&Resolution> {
        self.name_resolutions.get(id)
    }

    pub fn get_type_resolution(&self, id: &NodeId) -> Option<&CheckedTy> {
        self.type_resolutions.get(id)
    }
}
