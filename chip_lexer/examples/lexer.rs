use chip_lexer::lexer::Lexer;

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

    println!("{:?}", lexer.collect::<Vec<_>>())
}
