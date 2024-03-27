use proc_macro::TokenStream;
use quote::quote;
use syn;

// We’ve split the code into the hello_macro_derive function, which is responsible for parsing the TokenStream, and the impl_hello_macro function, which is responsible for transforming the syntax tree: this makes writing a procedural macro more convenient. 

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate

    let ast = syn::parse(input).unwrap();
    // Calling unwrap to cause the hello_macro_derive function to panic if the call to the syn::parse function fails here
    // It’s necessary for our procedural macro to panic on errors because proc_macro_derive functions must return TokenStream rather than Result

    // The code you specify in the body of the inner function (impl_hello_macro in this case) will be different depending on your procedural macro’s purpose.
    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // The quote! macro lets us define the Rust code that we want to return. The compiler expects something different to the direct result of the quote! macro’s execution, so we need to convert it to a TokenStream. 
    // The quote! macro also provides some very cool templating mechanics: we can enter #name, and quote! will replace it with the value in the variable name. You can even do some repetition similar to the way regular macros work.
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
                // The stringify! macro used here is built into Rust. It takes a Rust expression, such as 1 + 2, and at compile time turns the expression into a string literal, such as "1 + 2"
            }
        }
    };
    gen.into()
}