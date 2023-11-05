use chip_assembler::{assembler::Assembler, lexer::Lexer, parser::Parser};

fn main() {
    let lexer = Lexer::from(
        r#"
            ; Looping program that does nothing

            cls        ; Clear screen as NOP
            se  v0, v1
            se  v0, 20
            sne v0, v1
            sne v0, 20
            jmp 0x200  ; Jump back to the loop
        "#,
    );

    let parser = Parser::from(lexer);

    let assembler = Assembler::from(parser).collect::<Vec<u16>>();

    print_vec_as_hex(&assembler);

    loop {}
}

fn print_vec_as_hex(data: &Vec<u16>) {
    print!("[");
    for (index, value) in data.iter().enumerate() {
        if index > 0 {
            print!(", ");
        }
        print!("0x{:04X}", value);
    }
    println!("]");
}
