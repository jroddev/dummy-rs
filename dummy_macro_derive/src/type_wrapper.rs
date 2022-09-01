use syn::{self, Field};

#[derive(Debug)]
pub struct FieldParams {
    pub name: Option<syn::Ident>,
    pub data_type: syn::Ident,
}

#[derive(Debug)]
pub enum ConstructorType {
    Brace(),
    Parenthesis(),
}

#[derive(Debug)]
pub struct TypeWrapper {
    pub name: syn::Ident,
    pub fields: Vec<FieldParams>,
    pub constructor_type: ConstructorType,
}


pub fn variant_to_type_wrapper(data: &syn::Variant) -> TypeWrapper {
    let fields: Vec<Field> = match &data.fields {
        syn::Fields::Named(f) => f.named.clone().into_iter().collect(),
        syn::Fields::Unnamed(f) => f.unnamed.clone().into_iter().collect(),
        syn::Fields::Unit => Vec::new(),
    };
    TypeWrapper {
        name: data.ident.clone(),
        fields: fields.iter().map(get_field_type).collect(),
        constructor_type: match data.fields {
            syn::Fields::Named(_) => ConstructorType::Brace(),
            syn::Fields::Unnamed(_) => ConstructorType::Parenthesis(),
            syn::Fields::Unit => ConstructorType::Brace(),
        },
    }
}

pub fn struct_to_type_wrapper(type_name: syn::Ident, data: &syn::DataStruct) -> TypeWrapper {
    TypeWrapper {
        name: type_name,
        fields: data.fields.iter().map(get_field_type).collect(),
        constructor_type: match data.fields {
            syn::Fields::Named(_) => ConstructorType::Brace(),
            syn::Fields::Unnamed(_) => ConstructorType::Parenthesis(),
            syn::Fields::Unit => ConstructorType::Brace(),
        },
    }
}


pub fn union_to_type_wrapper(type_name: syn::Ident, data: &syn::DataUnion) -> TypeWrapper {
    TypeWrapper {
        name: type_name,
        fields: data.fields.named.iter().map(get_field_type).collect(),
        constructor_type: ConstructorType::Brace(),
    }
}

fn get_field_type(field: &Field) -> FieldParams {
    let name = field.ident.clone();
    let data_type = match &field.ty {
        syn::Type::Array(_) => todo!(),
        syn::Type::BareFn(_) => todo!(),
        syn::Type::Group(_) => todo!(),
        syn::Type::ImplTrait(_) => todo!(),
        syn::Type::Infer(_) => todo!(),
        syn::Type::Macro(_) => todo!(),
        syn::Type::Never(_) => todo!(),
        syn::Type::Paren(_) => todo!(),
        syn::Type::Path(p) => p.path.segments[0].ident.clone(),
        syn::Type::Ptr(_) => todo!(),
        syn::Type::Reference(_) => todo!(),
        syn::Type::Slice(_) => todo!(),
        syn::Type::TraitObject(_) => todo!(),
        syn::Type::Tuple(_) => todo!(),
        syn::Type::Verbatim(_) => todo!(),
        _ => todo!(),
    };
    FieldParams { name, data_type }
}
