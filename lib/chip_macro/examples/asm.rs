use chip_macro::asm;

fn main() {
    let program: &[u16] = asm!(
        "
          ; Looping program that does nothing

          cls       
          sne v0, 20     
          jmp 0x200      ; Jump back to loop
      "
    );

    println!("{:#?}", program);
}
