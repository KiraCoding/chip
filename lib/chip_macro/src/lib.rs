use chip_assembler::{assembler::Assembler, lexer::Lexer, parser::Parser};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn asm(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let asm_code = input.value();

    let lexer = Lexer::from(asm_code.as_str());
    let parser = Parser::from(lexer);

    let output = Assembler::from(parser).collect::<Vec<u16>>();

    quote! {
       &[#(#output), *]
    }
    .into()
}
