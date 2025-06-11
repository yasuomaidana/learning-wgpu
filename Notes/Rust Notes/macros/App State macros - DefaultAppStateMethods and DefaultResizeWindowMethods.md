First of all... You can see there is a crate called `state-derive`. Initially, that crate was created to contain `State` related default implementation, ~~but later I found that it was easy to include all my macros here... so I'm sorry, I was too lazy to create new crates later...~~ (fixed).

## Understanding Procedural Macros, `syn`, and `quote`

In Rust, procedural macros allow you to write Rust code that generates other Rust code. They are much more powerful than declarative macros (`macro_rules!`) because they operate on the Abstract Syntax Tree (AST) of your code.

- **`proc_macro`**: This is Rust's built-in crate for defining procedural macros. It provides the `TokenStream` type, which is the raw input and output of a procedural macro.
- **`syn`**: This crate is a _parser_. It takes a `TokenStream` (raw tokens) and parses it into a structured, traversable AST. This makes it easy to inspect and manipulate the Rust code you're trying to generate or transform. For example, `syn::DeriveInput` represents the structure of a `struct` or `enum` that a `#[derive]` macro is applied to.
- **`quote`**: This crate is a _code generator_. It allows you to construct `TokenStream`s from `syn` types and other Rust expressions in a very convenient and readable way, using a syntax that closely resembles regular Rust code. This is where the `#` interpolation magic happens.
## The `#` Symbol: Interpolation in `quote!`

The `#` symbol within `quote::quote! { ... }` (or `quote! { ... }` if `quote` is imported) is **not a general Rust syntax**. It is a special interpolation syntax provided by the `quote` crate. Its purpose is to inject Rust `TokenStream` fragments or `syn` types (which can be converted into `TokenStream`s) directly into the generated code.

Think of it like string interpolation in other languages, but for Rust code tokens.

**Why it's used:**

When you're writing a procedural macro, you often need to generate code that depends on the input type's name, its generics, or other parts of its structure. The `#` allows you to seamlessly insert these dynamic pieces of information into the static code template you're defining within `quote!`.

## Example
```rust
impl #impl_generics DefaultResizeWindowMethods for #name #ty_generics #where_clause {  
    ...
}
```

- `#impl_generics`: The `impl_generics` variable (which is a `syn::ImplGenerics` type) will be interpolated here, expanding to something like `<'a>` or `<T>` or `<'a, T: Trait>`.
- `#name`: The `name` variable (a `proc_macro2::Ident`) will be interpolated, becoming the actual name of the struct (e.g., `State`).
- `#ty_generics`: The `ty_generics` variable (a `syn::TypeGenerics`) will be interpolated, expanding to something like `<'a>` or `<T>` or `<'a, T>`.
- `#where_clause`: The `where_clause` variable (an `Option<&syn::WhereClause>`) will be interpolated, expanding to `where T: Debug` or nothing if no `where` clause is present.

> Essentially, `#` tells `quote!` to take the value of the variable following it and convert that value into a `TokenStream` fragment, inserting it at that point in the generated code.
---
## "Default/Reserved Names" for Generics in `syn`

When you parse a `DeriveInput` (which is what `#[derive]` macros operate on), the `syn::Generics` struct provides methods to easily extract different parts of the generic parameters for use in your `quote!` output. These aren't "reserved names" in the sense of keywords, but rather **conveniently structured components** that `syn` provides.

The most common way to get these components is by calling `generics.split_for_impl()`. This method returns a tuple of three parts, each designed for a specific position in an `impl` block:

1. **`impl_generics` (Type: `syn::ImplGenerics`)**:
    - **Purpose**: This represents the generic parameters that should appear immediately after the `impl` keyword. It includes lifetimes, type parameters, and `where` clause predicates.
    - **Example Expansion**: If your struct is `struct MyStruct<'a, T: Debug, const N: usize>`, `impl_generics` would expand to `<'a, T, N>` and potentially also include any necessary `where` clause predicates if they relate to the `impl` itself (though `split_for_impl` typically moves the main `where` clause to the end). It's primarily used to declare the generic parameters _for the implementation itself_.
2. **`ty_generics` (Type: `syn::TypeGenerics`)**:
	- **Purpose**: This represents the generic arguments that should appear after the type name in the `for` clause of an `impl` block or when referring to the type directly. It only includes the angle brackets and the names of the generic parameters (e.g., `<'a, T>`).
    - **Example Expansion**: For `MyStruct<'a, T: Debug, const N: usize>`, `ty_generics` would expand to `<'a, T, N>`. This is how you correctly refer to the generic instantiation of the type.
3. **`where_clause` (Type: `Option<&syn::WhereClause>`)**:
    - **Purpose**: This represents the `where` clause itself, if one exists for the input type. The `where` clause specifies additional constraints on the generic parameters.
    - **Example Expansion**: If your struct has `where T: MyTrait`, then `where_clause` would expand to `where T: MyTrait`. If there's no `where` clause, it will expand to nothing (`None`).
### How to use:
```rust
impl #impl_generics TraitName for TypeName #ty_generics #where_clause {
    // ...
}
```
This structure is a standard pattern for generating `impl` blocks in procedural macros because `syn` designed `split_for_impl()` to provide exactly the pieces needed for this common syntax.

## Usage example
Traits
```rust
pub trait DefaultResizeWindowMethods {  
    fn resize(&mut self, new_size: PhysicalSize<u32>);  
    fn window(&self) -> &Window;  
}  
  
pub trait DefaultAppStateMethods: DefaultResizeWindowMethods {  
    fn update(&mut self);  
}

pub trait AppState: DefaultAppStateMethods {  
    fn new(window: Window) -> Self;  
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;  
}
```
Procedure macro usage
```rust
#[derive(DefaultAppStateMethods)]  
pub struct State<'a> {  
    surface: Surface<'a>,  
    config: wgpu::SurfaceConfiguration,  
    size: PhysicalSize<u32>,  
    window: Arc<Window>,  
    toggled: bool,  
    // ... 
    // other fields
    // ...
}
// Render is impleneted using a trait
```

1. When the Rust compiler encounters `#[derive(DefaultResizeWindowMethods)]`, it invokes the `default_resize_window_methods` procedural macro function.
2. The macro receives the `TokenStream` representing your `State` struct.
3. Inside `default_resize_window_methods`:
    - `syn::parse(item).unwrap()` parses the `TokenStream` into a `syn::DeriveInput` AST. This `ast` now contains structured information about `State`.
    - `let name = &abstract_syntax_tree.ident;` extracts `State` as a `proc_macro2::Ident`.
    - `let generics = &abstract_syntax_tree.generics;` extracts the `Generics` struct, which for `State<'a>` would contain the lifetime parameter `'a`.
    - `let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();` populates these three variables:
        - `impl_generics` would likely be `<'a>`
        - `ty_generics` would likely be `<'a>`
        - `where_clause` would be `None` (as there's no explicit `where` clause on `State`)
    - `generate_default_resize_window_methods_impl(...)` is called with these pieces.
    - Inside `generate_default_resize_window_methods_impl`, the `quote!` macro uses the `#` interpolation.
### Expansion
4. The `quote!` macro generates the `TokenStream` that looks like this:
```rust

impl<'a> DefaultAppStateMethods for State<'a> {
	fn update(&mut self) {
		self.toggled = !self.toggled;
		self.render().unwrap();
	}
}

impl<'a> DefaultResizeWindowMethods for State<'a> {
	fn resize(&mut self, new_size: PhysicalSize<u32>) {
		self.size = new_size;
		self.config.width = new_size.width;
		self.config.height = new_size.height;
		self.surface.configure(&self.device, &self.config);
	}
	fn window(&self) -> &winit::window::Window { &self.window }
}
```
5. This generated `TokenStream` is then returned by the procedural macro and effectively inserted into your code during compilation, as if you had written that `impl` block manually.

This entire process is why your `State` struct (or any other struct you `derive` this macro on) automatically gets the `resize` and `window` methods without you having to write the `impl` block yourself.