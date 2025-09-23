use super::cfg::Cfg;

pub struct LivenessAnalysis<'a> {
    cfgs: &'a [Cfg],
}

impl<'a> LivenessAnalysis<'a> {
    pub fn new() -> Self {}
}
