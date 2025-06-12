use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, Data, Fields, Type};

fn generate_default_app_impl(
    name: &proc_macro2::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    event_handler_ty: &Type,
) -> proc_macro2::TokenStream {
    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn new(window_name: String) -> Self {
                Self {
                    state: None,
                    event_handler: <#event_handler_ty>::new(),
                    window_name,
                }
            }
        }
    }
}

#[proc_macro_derive(DefaultApp)]
pub fn default_app_derive(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    // Find the event_handler field type
    let mut event_handler_ty = None;
    if let Data::Struct(data_struct) = &ast.data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in &fields.named {
                if field.ident.as_ref().map(|i| i == "event_handler").unwrap_or(false) {
                    event_handler_ty = Some(&field.ty);
                    break;
                }
            }
        }
    }
    let event_handler_ty = event_handler_ty.expect("No event_handler field found");

    generate_default_app_impl(name, &impl_generics, &ty_generics, where_clause,event_handler_ty).into()
}
