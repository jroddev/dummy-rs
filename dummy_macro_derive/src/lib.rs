use proc_macro::{TokenStream, Ident};
use quote::quote;
use syn::{self, Field};
extern crate proc_macro;

/*
    Group::delimiter::Brace = { }
    Group::delimiter::Parenthesis = ( )
    I can use these to determine dynamically which style needs to be used
*/

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

fn build_enum(name: &syn::Ident, e: &syn::DataEnum) -> TokenStream {
    let variant_count = e.variants.len() as u32;

    let mut index: u32 = 0;
    let quoted: Vec<_> = e.variants.iter().map(|v|{
        let name = &v.ident;
        println!("FIELDS: {:?}", &v.fields);
        let fields = fields_blah(&v.fields);
        let variant_line = match &v.fields {
            syn::Fields::Named(_) => {
                quote! {
                    #index => Self::#name {
                        #(#fields,)*
                    }
                }
            },
            syn::Fields::Unnamed(_) => {
                quote! {
                    #index => Self::#name (
                        #(#fields,)*
                    )
                }
            },
            syn::Fields::Unit => {
                quote! {#index => Self::#name}
            }
        };


        // let variant_line = quote! {
        //     #index => Self::#name (
        //         #(#fields,)*
        //     )
        // };
        // let variant_line = quote! {
        //     #index => Self::#name{}
        // };
        index += 1;
        variant_line
    }).collect();

    quote! {
        impl Dummy for #name {
            fn dummy() -> Self {
                let variant_id = random::<u32>()%#variant_count;
                match variant_id {
                   #(#quoted,)*
                    // _ => panic!("Dummy Enum Variant {} Out of Bounds: {}", #name, variant_id)
                    _ => todo!()
                }
            }
        }
    }.into()
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

fn fields_blah(fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    let unwrapped : Vec<_> = match fields {
        syn::Fields::Named(f) => f.named.iter().collect(),
        syn::Fields::Unnamed(f) => f.unnamed.iter().collect(),
        syn::Fields::Unit => Vec::new(),
    };

    let output = unwrapped.iter().map(|f|{
        let ft = get_field_type(f);
        field_to_dummy_call(&ft)
    }).collect();
    output
}

fn field_to_dummy_call(fp: &FieldParams) -> proc_macro2::TokenStream {
    let data_type = &fp.data_type;
    match &fp.name {
        // Some(name) => quote!{#name: 1},
        // None => quote!{1}
        Some(name) => quote!{#name: #data_type::dummy()},
        None => quote!{#data_type::dummy()}
    }
}

enum BracketStyle {
    ConstructorStyle,   // { and }
    FunctionStyle       // ( and )
}

fn impl_dummy_macro(ast: &syn::DeriveInput) -> TokenStream {
    println!("ast:{:?}", ast);
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
            return build_enum(&ast.ident, e).into()
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

    let quoted: Vec<_> = lines.iter().map(|line| {
        // let data_type = &line.data_type;
        // match &line.name {
        //     Some(name) => quote!{#name: #data_type::dummy()},
        //     None => quote!{#data_type::dummy()}
        // }
        field_to_dummy_call(line)
    }).collect();
    println!("quoted: {:?}", quoted);

    let name = &ast.ident;

    let gen = match bracket_style {
        // TODO: see note at top for Brace / Parenthesis
        BracketStyle::ConstructorStyle => {
            quote! {
                impl Dummy for #name {
                    fn dummy() -> Self {
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
                    fn dummy() -> Self {
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
