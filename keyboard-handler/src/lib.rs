use proc_macro2::TokenStream;
use syn::DeriveInput;

/// Generates an implementation of the `DefaultKeyboardHandlerMethods` trait
/// for the given type, using the provided generics and where clause.
///
/// # Parameters
/// - `name`: The identifier of the type to implement the trait for.
/// - `ty_generics`: The type generics for the type.
/// - `where_clause`: An optional where clause for the implementation.
/// - `impl_generics`: The generics for the implementation block.
///
/// # Returns
/// A `TokenStream` containing the generated implementation.
fn impl_default_keyboard_handler(
    name: &proc_macro2::Ident,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    impl_generics: &syn::ImplGenerics,
    action_ty: &syn::Type,
) -> TokenStream {
    quote::quote! {
        impl #impl_generics DefaultKeyboardHandlerMethods for #name #ty_generics #where_clause {
            type Action = #action_ty;

            fn input(&mut self, event: &winit::event::WindowEvent) {
                self.actions.iter_mut().for_each(|action| action.update(event));
            }
            fn clear(&mut self) {
                self.actions.iter_mut().for_each(|action| action.clear());
            }
        }
    }
}

#[proc_macro_derive(DefaultKeyboardHandler, attributes(action_type))]
pub fn derive_default_keyboard_handler(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let abstract_syntax_tree: DeriveInput = syn::parse(input).unwrap(); // Conventionally named `ast`

    let action_ty = abstract_syntax_tree
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("action_type"))
        .and_then(|attr| attr.parse_args().ok())
        .expect("You must provide #[action_type(TypeName)] attribute");

    // Generate the implementation of the DefaultResizeWindowMethods trait
    let name = &abstract_syntax_tree.ident;
    let generics = &abstract_syntax_tree.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    impl_default_keyboard_handler(name, &ty_generics, where_clause, &impl_generics, &action_ty)
        .into()
}
