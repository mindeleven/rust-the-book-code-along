/// Macros
/// (1) declarative macros with macro_rules! 
/// (2) three kinds of procedural macros:
/// (2/1) Custom #[derive] macros that specify code added with the derive attribute
/// (2/2) Attribute-like macros that define custom attributes usable on any item
/// (2/3) Function-like macros that look like function calls 
///       but operate on the tokens specified as their argument
/// 
/// (1) Declarative Macros with macro_rules! for General Metaprogramming
/// declarative macros allow to write something that's similar to a Rust match expression
/// a macro is defined with the macro_rules! construct
/// example: slightly simplified definition of the vec! macro
/// 
#[macro_export]
macro_rules! vec_cust {
    // one arm with the pattern ( $( $x:expr ),* )
    // $x:expr matches any Rust expression and gives the expression the name $x
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// (2/2) Procedural Macros for Generating Code from Attributes
/// procedural macros accept some code as an input, 
/// operate on that code, 
/// and produce some code as an output

fn main() {
    // calling declarative Macro example
    let my_vec = vec_cust![21, 22, 34];
    println!("{:?}", my_vec);
}
