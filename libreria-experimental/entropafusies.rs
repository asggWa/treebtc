##[macro_use]
extern crate static_assertions;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

//use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
sa::const_assert!(true); ///CUIDADO OVER DIAG
let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())


/*
TokenStream requires set get?  
#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    input=<? echo"Hecho test" info.php; ?>
    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
let mut a int;

    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
*/
//Finaliza el main. Test prinln
//println!("Hola Web3");
}

