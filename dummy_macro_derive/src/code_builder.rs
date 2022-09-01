extern crate proc_macro;
use quote::quote;
use crate::type_wrapper::*;


pub fn field_to_dummy_call(fp: &FieldParams) -> proc_macro2::TokenStream {
    let data_type = &fp.data_type;
    match &fp.name {
        Some(name) => quote! {#name: #data_type::dummy()},
        None => quote! {#data_type::dummy()},
    }
}

pub fn construct_dummy_type(input: TypeWrapper) -> proc_macro2::TokenStream {
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


pub fn build_enum(name: &syn::Ident, e: &syn::DataEnum) -> proc_macro2::TokenStream {
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
