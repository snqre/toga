#![allow(clippy::redundant_field_names)]
#![allow(clippy::let_with_type_underscore)]

mod impls;

/// Procedural macro for generating `impl` blocks with inherent methods and trait implementations.
/// 
/// The `blockset!` macro allows defining inherent methods and trait implementations for a type
/// in a single, cohesive syntax block.
/// 
/// # Syntax
/// 
/// ```text
/// impl<ImplGenerics> Type<TypeGenerics>
/// where
///     ...;
/// 
/// fn method1(...) { ... }
/// fn method2(...) -> T { ... }
/// 
/// path::ToTrait<TraitGenerics> {
///     fn trait_method(...) -> ... { ... }
/// }
/// ```
/// 
/// - The `impl` line defines the target type, optionally with generics and a `where` clause.
/// - Any `fn` blocks after the header are treated as inherent methods.
/// - Any block with a `trait_path { ... }` format is interpreted as a trait implementation block.
/// 
/// # Example
/// 
/// ```rust
/// use toga::impls;
/// 
/// trait Health<T> {
///     fn health(&self) -> T;
/// }
/// 
/// trait Wizard {
///     fn you_shall_not_pass(&self) {}
/// }
/// 
/// struct Player<const A: usize, B>(B);
/// 
/// impls! {
///     impl<const A: usize, B> Player<A, B>
///     where
///         B: Clone;
/// 
///     pub fn hello_world(&self) {}
/// 
///     pub fn give_me_a_number(&self) -> u8 {
///         50
///     }
/// 
///     Wizard {}
/// 
///     Health<u8> {
///         fn health(&self) -> u8 {
///             100
///         }
///     }
/// }
/// ```
/// 
/// # Notes
/// 
/// - All generics and where clauses are applied to both the inherent and trait impls.
/// - The trait paths must be valid and resolvable in scope (e.g., `self::TraitName` or `crate::TraitName`).
/// - Multiple trait blocks and methods are supported.
#[proc_macro]
pub fn impls(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(stream as impls::Impls).resolve().into()
}