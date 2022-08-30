use proc_macro::{TokenStream, Ident};
use quote::{quote, ToTokens};
use syn::{self, Field};

#[proc_macro_derive(Dummy)]
pub fn dummy_macro_derive(input: TokenStream) -> TokenStream {

    println!("token stream: {:?}", input);
    let ast= syn::parse(input).unwrap();
    println!("----");

    impl_dummy_macro(&ast)
}

struct FieldParams {
    name: Option<syn::Ident>,
    data_type: syn::Ident
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


fn impl_dummy_macro(ast: &syn::DeriveInput) -> TokenStream {
    let mut lines = Vec::new();
    match &ast.data {
        syn::Data::Struct(s) => {
            match &s.fields {
                syn::Fields::Named(f) => {
                    for param in &f.named { lines.push(get_field_type(param)) }
                }
                syn::Fields::Unnamed(f) => {
                    for param in &f.unnamed { lines.push(get_field_type(param)) }
                },
                syn::Fields::Unit => {},
            }
        }
        syn::Data::Union(u) => {
            for param in &u.fields.named {
                lines.push(get_field_type(param));
            }
        },
        syn::Data::Enum(_) => todo!(),
    };

    let quoted = lines.iter().map(|line| {
        let data_type = &line.data_type;
        match &line.name {
            Some(name) => quote!{#name: random::<#data_type>()},
            None => quote!{random::<#data_type>()}
        }
    });

    let name = &ast.ident;
    let gen = quote! {
        impl Dummy for #name {
            fn dummy() -> #name {
                #name {
                   #(#quoted,)*
                }
            }
        }
    };
    gen.into()
}
