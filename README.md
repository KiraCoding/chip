# Chip

> [!IMPORTANT]\
> Chip is currently under development, and official releases have not been shipped yet.

A development suite for CHIP8 virtual machine enthusiasts, offering essential tools for creating, assembling, and interpreting retro-style games and applications.

| Crate              | Description                                |
| :----------------- | :----------------------------------------- |
| [chip]             | Whole CHIP-8 toolchain.                    |
| [chip_assembler]   | Assembler for CHIP-8 assembly language.    |
| [chip_interpreter] | Interpreter for executing CHIP-8 programs. |
| [chip_lexer]       |                                            |
| [chip_macro]       | Utility macros                             |

## Building

### Prerequisites
- `Rust >= 1.64.0`

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

[chip]:             ./chip/
[chip_assembler]:   ./chip_assembler/
[chip_interpreter]: ./chip_interpreter/
[chip_lexer]:       ./chip_lexer/
[chip_macro]:       ./chip_macro/
[license]:          ./LICENSE
