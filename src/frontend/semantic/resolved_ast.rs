use super::resolved_decl::ResolvedDecl;

pub struct ResolvedAst {
    pub main_function_id: usize,
    pub declarations: Vec<ResolvedDecl>,
}

impl ResolvedAst {
    pub fn new(main_function_id: usize, declarations: Vec<ResolvedDecl>) -> Self {
        Self {
            main_function_id,
            declarations,
        }
    }
}
