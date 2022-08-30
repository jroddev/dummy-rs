use proc_macro::{TokenStream, Ident};
use quote::{quote, ToTokens};
use syn::{self, Field};
// use rand::seq::SliceRandom;

#[proc_macro_derive(Dummy)]
pub fn dummy_macro_derive(input: TokenStream) -> TokenStream {

    println!("token stream: {:?}", input);
    let ast= syn::parse(input).unwrap();
    println!("----");

    impl_dummy_macro(&ast)
}

#[derive(Debug)]
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

enum BracketStyle {
    ConstructorStyle,   // { and }
    FunctionStyle       // ( and )
}

fn impl_dummy_macro(ast: &syn::DeriveInput) -> TokenStream {
    let mut lines = Vec::new();
    let mut bracket_style = BracketStyle::ConstructorStyle;
    match &ast.data {
        syn::Data::Struct(s) => {
            match &s.fields {
                syn::Fields::Named(f) => {
                    println!("struct::named");
                    for param in &f.named { lines.push(get_field_type(param)) }
                }
                syn::Fields::Unnamed(f) => {
                    println!("struct::unnamed");
                    // TODO: this needs round braces not curvy
                    // Tuple3(1, 2, 3) instead of Tuple3{1,2,3}
                    for param in &f.unnamed {
                        println!("field: {:?}", param);
                        lines.push(get_field_type(param))
                    }
                    bracket_style = BracketStyle::FunctionStyle;
                },
                syn::Fields::Unit => {
                    println!("struct::unit");
                },
            }
        }
        syn::Data::Union(u) => {
            println!("union");
            for param in &u.fields.named {
                lines.push(get_field_type(param));
            }
        },
        syn::Data::Enum(e) => {
            println!("enum");
            // I think I need to 'choose' in the generated function
            /*
                impl Dummy for MyEnum {
                    fn dummy() -> MyEnum {
                        let variant = <logic>
                        match variant {
                            case VariantA => VariantA(),
                            case VariantB => VariantB(dummy::<i32>()),
                            case VariantC => VariantC(dummy::<String>(), dummy::<i32>()),
                        }
                    }
                }
            */
            // let mut rng = rand::thread_rng();
            // let variants = e.variants
            //     .clone()
            //     .into_iter()
            //     .collect::<Vec<_>>();
            // let variant = variants.choose(&mut rng).unwrap();
            // println!("enum variant: {:?}", variant);
            // for param in &variant.fields { lines.push(get_field_type(param)) }
        },
    };

    println!("lines: {:?}", lines);

    let quoted = lines.iter().map(|line| {
        let data_type = &line.data_type;
        match &line.name {
            Some(name) => quote!{#name: #data_type::dummy()},
            None => quote!{#data_type::dummy()}
        }
    });
    println!("quoted: {:?}", quoted);

    let name = &ast.ident;

    let gen = match bracket_style {
        BracketStyle::ConstructorStyle => {
            quote! {
                impl Dummy for #name {
                    fn dummy() -> #name {
                        #name {
                           #(#quoted,)*
                        }
                    }
                }
            }
        },
        BracketStyle::FunctionStyle => {
            quote! {
                impl Dummy for #name {
                    fn dummy() -> #name {
                        #name (
                           #(#quoted,)*
                        )
                    }
                }
            }
        }
    };

    println!("generated: {}", gen);
    gen.into()
}
