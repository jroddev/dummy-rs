use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Dummy)]
pub fn dummy_macro_derive(input: TokenStream) -> TokenStream {

    println!("token stream: {:?}", input);
    let ast= syn::parse(input).unwrap();
    println!("----");

    impl_dummy_macro(&ast)
}

fn impl_dummy_macro(ast: &syn::DeriveInput) -> TokenStream {
    let mut lines = Vec::new();
    match &ast.data {
        syn::Data::Struct(s) => {
            match &s.fields {
                syn::Fields::Named(f) => {
                    for param in &f.named { 
                        let name = &param.ident.clone().unwrap();
                        let field = match &param.ty {
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

                        println!("f {:?} |||| {:?}", name, field);
                        lines.push(
                            quote!{#name: random::<#field>()}
                            );
                    }
                }
                syn::Fields::Unnamed(_) => todo!(),
                syn::Fields::Unit => todo!(),
            }
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };

    println!("lines: {:?}", lines);

    let name = &ast.ident;
    let gen = quote! {
        impl Dummy for #name {
            fn dummy() -> #name {
                #name {
                   #(#lines,)* 
                }
            }
        }
    };
    gen.into()
}
