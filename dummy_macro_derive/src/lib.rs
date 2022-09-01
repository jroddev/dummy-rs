use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::{self, Field};
extern crate proc_macro;

#[proc_macro_derive(Dummy)]
pub fn dummy_macro_derive(input: TokenStream) -> TokenStream {
    println!("token stream: {:?}", input);
    let ast = syn::parse(input).unwrap();
    println!("----");

    impl_dummy_macro(&ast).into()
}

#[derive(Debug)]
struct FieldParams {
    name: Option<syn::Ident>,
    data_type: syn::Ident,
}

#[derive(Debug)]
enum ConstructorType {
    Brace(),
    Parenthesis(),
}

#[derive(Debug)]
struct TypeWrapper {
    name: syn::Ident,
    fields: Vec<FieldParams>,
    constructor_type: ConstructorType,
}

fn construct_dummy_type(input: TypeWrapper) -> proc_macro2::TokenStream {
    let name = input.name;
    let fields: Vec<proc_macro2::TokenStream> =
        input.fields.iter().map(field_to_dummy_call).collect();
    match input.constructor_type {
        ConstructorType::Brace() => {
            quote! {
                #name { #(#fields,)* }
            }
        }
        ConstructorType::Parenthesis() => {
            quote! {
                #name ( #(#fields,)* )
            }
        }
    }
}

fn field_to_dummy_call(fp: &FieldParams) -> proc_macro2::TokenStream {
    let data_type = &fp.data_type;
    match &fp.name {
        Some(name) => quote! {#name: #data_type::dummy()},
        None => quote! {#data_type::dummy()},
    }
}

fn variant_to_type_wrapper(data: &syn::Variant) -> TypeWrapper {
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

fn struct_to_type_wrapper(type_name: syn::Ident, data: &syn::DataStruct) -> TypeWrapper {
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


fn union_to_type_wrapper(type_name: syn::Ident, data: &syn::DataUnion) -> TypeWrapper {
    TypeWrapper {
        name: type_name,
        fields: data.fields.named.iter().map(get_field_type).collect(),
        constructor_type: ConstructorType::Brace(),
    }
}

fn build_enum(name: &syn::Ident, e: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variant_count = e.variants.len() as u32;

    let mut index: u32 = 0;
    let quoted: Vec<_> = e
        .variants
        .iter()
        .map(|v| {
            let wrapper = variant_to_type_wrapper(v);
            let construct_variant = construct_dummy_type(wrapper);
            let variant_line = quote! {
                #index => Self::#construct_variant
            };

            index += 1;
            variant_line
        })
        .collect();

    let enum_str = name.to_string();
    quote! {
        let enum_str = #enum_str;
        let variant_id = random::<u32>() % #variant_count;
        match variant_id {
           #(#quoted,)*
            _ => panic!("Dummy variant {} out of bounds: {}", enum_str, variant_id)
        }
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

fn impl_dummy_macro(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    println!("ast:{:?}", ast);
    let name = &ast.ident;
    let implementation = match &ast.data {
        syn::Data::Struct(s) => {
            let wrapper = struct_to_type_wrapper(name.clone(), s);
            construct_dummy_type(wrapper)
        }
        syn::Data::Enum(e) => {
            build_enum(&ast.ident, e)
        }
        syn::Data::Union(u) => {
            let wrapper = union_to_type_wrapper(name.clone(), u);
            construct_dummy_type(wrapper)
        }
    };

    let gen = quote! {
        impl Dummy for #name {
            fn dummy() -> Self {
                #implementation
            }
        }
    };

    println!("generated: {}", gen);
    gen.into()
}
