#[proc_macro_derive(DeriveInitError)]
pub fn derive_init_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("DeriveInitError syn::parse(input) failed");
    let fields = match ast.data {
        syn::Data::Struct(struct_item) => struct_item.fields,
        _ => panic!("DeriveInitError only works on structs"),
    };
    let ident = &ast.ident;
    let source_type_ident = match fields {
        syn::Fields::Named(fields_named) => {
            match fields_named.named.len() {
                2 => {
                    match &fields_named.named[0].ty {
                        // syn::Type::Array(_) => todo!(),
                        // syn::Type::BareFn(_) => todo!(),
                        // syn::Type::Group(_) => todo!(),
                        // syn::Type::ImplTrait(_) => todo!(),
                        // syn::Type::Infer(_) => todo!(),
                        // syn::Type::Macro(_) => todo!(),
                        // syn::Type::Never(_) => todo!(),
                        // syn::Type::Paren(_) => todo!(),
                        // syn::Type::Ptr(_) => todo!(),
                        // syn::Type::Reference(_) => todo!(),
                        // syn::Type::Slice(_) => todo!(),
                        // syn::Type::TraitObject(_) => todo!(),
                        // syn::Type::Tuple(_) => todo!(),
                        // syn::Type::Verbatim(_) => todo!(),
                        syn::Type::Path(type_path) => {
                            if type_path.path.segments.len() != 1 {
                                panic!(
                                    "DeriveStructFieldSetter type_path.path.segments != 1, length is {}",
                                    type_path.path.segments.len()
                                );
                            }
                            type_path.path.segments[0].ident.clone()
                        }
                        _ => panic!("DeriveStructFieldSetter only works on structs fields with  syn::Type::Path type"),
                    }
                }
                _ => panic!("DeriveInitError fields_named.len() != 2"),
            }
        }
        // syn::Fields::Unnamed(_) => todo!(),
        // syn::Fields::Unit => todo!(),
        _ => panic!("DeriveInitError only works with named fields"),
    };
    let gen = quote::quote! {
        impl #ident {
            pub fn new(source: #source_type_ident, where_was: Vec<crate::helpers::where_was::WhereWasOneOrFew>) -> Self {
                Self { source, where_was }
            }
        }
    };
    gen.into()
}
