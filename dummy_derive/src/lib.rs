use proc_macro::TokenStream;
use quote::quote;

mod type_wrapper;
mod code_builder;

#[proc_macro_derive(Dummy)]
pub fn dummy_macro_derive(input: TokenStream) -> TokenStream {
    // println!("token stream: {:?}", input);
    let ast = syn::parse(input).unwrap();
    // println!("----");

    impl_dummy_macro(&ast).into()
}


fn impl_dummy_macro(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    // println!("ast:{:?}", ast);
    let name = &ast.ident;
    let implementation = match &ast.data {
        syn::Data::Struct(s) => {
            let wrapper = type_wrapper::struct_to_type_wrapper(name.clone(), s);
            code_builder::construct_dummy_type(wrapper)
        }
        syn::Data::Enum(e) => {
            code_builder::build_enum(&ast.ident, e)
        }
        syn::Data::Union(u) => {
            let wrapper = type_wrapper::union_to_type_wrapper(name.clone(), u);
            code_builder::construct_dummy_type(wrapper)
        }
    };

    let gen = quote! {
        impl Dummy for #name {
            fn dummy() -> Self {
                #implementation
            }
        }
    };

    // println!("generated: {}", gen);
    gen
}
