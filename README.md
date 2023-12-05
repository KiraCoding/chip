# Chip

> [!IMPORTANT]\
> Chip is currently under development, and official releases have not been shipped yet.

A development suite for CHIP8 virtual machine enthusiasts, offering essential tools for creating, assembling, and interpreting retro-style games and applications.

| Crate              | Description                                | Binary | Library |
| :----------------- | :----------------------------------------- | :----: | :-----: |
| [chip]             | Whole CHIP-8 toolchain.                    | X      | X       |
| [chip_assembler]   | Assembler for CHIP-8 assembly language.    |        | X       |
| [chip_interpreter] | Interpreter for executing CHIP-8 programs. |        | X       |
| [chip_macro]       | Utility macros                             |        | X       |

## Building

### Requirements
```
Rust ─ >= 1.64.0
```

### Debug
```
cargo build
```

### Release
```
cargo build --release
```

## License
Chip is distributed under the terms of the GNU General Public License v3.0.
- [`LICENSE`][license] ─ `GNU GPL-3.0` ─ https://www.gnu.org/licenses/gpl-3.0.en.html

[chip]:             ./bin/chip/ 
[chip_assembler]:   ./lib/chip_assembler/
[chip_interpreter]: ./lib/chip_interpreter/
[chip_macro]:       ./lib/chip_macro/
[license]:          ./LICENSE
