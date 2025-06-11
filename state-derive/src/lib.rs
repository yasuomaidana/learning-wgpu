use proc_macro::TokenStream;
use syn::DeriveInput;

/// Generates an implementation of the `DefaultResizeWindowMethods` trait for the given type.
///
/// # Arguments
/// * `name` - The identifier of the type to implement the trait for.
/// * `ty_generics` - The type generics for the type.
/// * `where_clause` - An optional where clause for the implementation.
/// * `impl_generics` - The implementation generics for the type.
///
/// # Returns
/// A `TokenStream` containing the generated implementation.
fn generate_default_resize_window_methods_impl(
    name: &proc_macro2::Ident,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    impl_generics: &syn::ImplGenerics,
) -> proc_macro2::TokenStream {
    quote::quote! {
        // In Rust macros (especially with quote::quote!), the # symbol is used for interpolation.
        // It inserts the value of a variable or expression into the generated code.
        // In your selected lines, #impl_generics, #name, #ty_generics, and #where_clause
        // are replaced with their actual values when the macro is expanded,
        // allowing the macro to generate code that adapts to the input type
        // and its generics.
        impl #impl_generics DefaultResizeWindowMethods for #name #ty_generics #where_clause {
            fn resize(&mut self, new_size: PhysicalSize<u32>) {
                self.size = new_size;

                self.config.width = new_size.width;
                self.config.height = new_size.height;

                self.surface.configure(&self.device, &self.config);
            }
            fn window(&self) -> &winit::window::Window {
                &self.window
            }
        }
    }
}

/// Generates an implementation of the `DefaultAppStateMethods` trait for the given type.
///
/// # Arguments
/// * `name` - The identifier of the type to implement the trait for.
/// * `ty_generics` - The type generics for the type.
/// * `where_clause` - An optional where clause for the implementation.
/// * `impl_generics` - The implementation generics for the type.
///
/// # Returns
/// A `TokenStream` containing the generated implementation.
fn generate_default_app_state_methods_impl(
    name: &proc_macro2::Ident,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    impl_generics: &syn::ImplGenerics,
) -> proc_macro2::TokenStream {
    let default_resize_window =
        generate_default_resize_window_methods_impl(name, ty_generics, where_clause, impl_generics);
    quote::quote! {
        impl #impl_generics DefaultAppStateMethods for #name #ty_generics #where_clause {

            fn update(&mut self) {
                // Update the state of the application
                self.toggled = !self.toggled;
                self.render().unwrap();
            }
        }
        #default_resize_window
    }
}

/// Generates the implementation of the `DefaultAppStateMethods` trait for the given type
/// by analyzing the provided `DeriveInput` AST node.
///
/// # Arguments
/// * `ast` - The syntax tree representing the type to implement the trait for.
///
/// # Returns
/// A `TokenStream` containing the generated implementation.
fn impl_default_app_state_methods(ast: &DeriveInput) -> TokenStream {
    // This function is a placeholder for the actual implementation
    // that will generate the methods for the DefaultAppStateMethods trait.
    // The implementation will depend on the structure of the `ast` and
    // the specific requirements of the trait.
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    generate_default_app_state_methods_impl(&name, &ty_generics, where_clause, &impl_generics)
        .into()
}

/// Procedural macro to automatically implement the `DefaultAppStateMethods` trait
/// for a struct. This macro parses the input type and generates the required trait
/// implementation, allowing the struct to handle application state logic by default.
///
/// # Arguments
/// * `item` - The input token stream representing the struct to derive the trait for.
///
/// # Returns
/// A `TokenStream` containing the generated implementation.
#[proc_macro_derive(DefaultAppStateMethods)]
pub fn default_app_state_methods_macro(item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree

    // The syn::parse function attempts to convert the token stream into
    // a structured representation of Rust code. The unwrap() call is used
    // to panic if parsing fails, which is common in macros for simplicity
    // during development.
    let abstract_syntax_tree: DeriveInput = syn::parse(item).unwrap(); // Conventionally named `ast`
    // Generate the implementation of the DefaultAppStateMethods trait
    impl_default_app_state_methods(&abstract_syntax_tree)
}

/// Procedural macro to automatically implement the `DefaultResizeWindowMethods` trait
/// for a struct. This macro parses the input type and generates the required trait
/// implementation, allowing the struct to handle window resizing logic by default.
///
/// # Example
/// ```rust
/// use state_derive::DefaultResizeWindowMethods;
///
/// #[derive(DefaultResizeWindowMethods)]
/// struct MyWindowState { /* fields */ }
/// ```
#[proc_macro_derive(DefaultResizeWindowMethods)]
pub fn default_resize_window_methods(item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let abstract_syntax_tree: DeriveInput = syn::parse(item).unwrap(); // Conventionally named `ast`
    // Generate the implementation of the DefaultResizeWindowMethods trait
    let name = &abstract_syntax_tree.ident;
    let generics = &abstract_syntax_tree.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    generate_default_resize_window_methods_impl(&name, &ty_generics, where_clause, &impl_generics)
        .into()
}
