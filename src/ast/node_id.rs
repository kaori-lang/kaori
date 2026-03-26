use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId(Uuid);

impl Default for NodeId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}
