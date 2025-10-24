use std::collections::HashMap;

use super::hir_id::HirId;

#[derive(Debug, Default)]
pub struct Types {
    types: HashMap<HirId, Type>,
}

impl Types {
    pub fn get(&self, id: HirId) -> Type {
        let ty = self.types.get(&id).unwrap();

        self.resolve_type(ty)
    }

    pub fn insert(&mut self, id: HirId, ty: Type) {
        self.types.insert(id, ty);
    }

    fn resolve_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Boolean => Type::Boolean,
            Type::String => Type::String,
            Type::Number => Type::Number,
            Type::Void => Type::Void,
            Type::Function {
                parameters,
                return_ty,
            } => {
                let parameters = parameters.iter().map(|ty| self.resolve_type(ty)).collect();

                let return_ty = self.resolve_type(return_ty);

                Type::function(parameters, return_ty)
            }

            Type::Struct { fields } => {
                let fields = fields
                    .iter()
                    .map(|field| self.resolve_type(field))
                    .collect();

                Type::struct_(fields)
            }

            Type::TypeRef(id) => self.get(*id),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum Type {
    Boolean,
    String,
    Number,
    #[default]
    Void,
    Function {
        parameters: Vec<Type>,
        return_ty: Box<Type>,
    },
    Struct {
        fields: Vec<Type>,
    },
    TypeRef(HirId),
}

impl Type {
    pub fn function(parameters: Vec<Type>, return_ty: Type) -> Type {
        Type::Function {
            parameters,
            return_ty: Box::new(return_ty),
        }
    }

    pub fn struct_(fields: Vec<Type>) -> Type {
        Type::Struct { fields }
    }

    pub fn type_ref(id: HirId) -> Type {
        Type::TypeRef(id)
    }
}
