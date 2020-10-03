extern crate proc_macro;
use proc_macro::*;


#[proc_macro_attribute]
pub fn destination_queue(args: TokenStream, input: TokenStream) -> TokenStream {
    // "pub struct MusicEvent{
    //     pub artists: Vec<String>,
    //     pub location: String
    // }
    
    // impl MusicEvent{
    //     fn get_name() -> &str {
    //         \"queueueueueueu\"
    //     }
    // }
    // ".parse().unwrap()
    input
}