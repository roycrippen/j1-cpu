# J1 CPU emulator

[J1](https://excamera.com/sphinx/fpga-j1.html) CPU emulator written in Rust. <br>
Will be migrated to a variant of [J1-hacked](https://www.fpgarelated.com/showarticle/790.php) once completed. <br>
Ported from [j1](https://github.com/dim13/j1) written in go. <br>
requires [rustup](https://rustup.rs/) <br>


### build
```shell
cargo build --release
```

### test
```shell
cargo test
```

### run j1 forth repl
```shell
./target/release/j1_repl
```


### todo
| Task              | Done |
| :---------------  | :------: |
| stack             | &#x2714; |
| instructions      | &#x2714; |
| cpu               | &#x2714; |
| dump bin to asm   |  |
| j1 forth emulator | &#x2714; |
| args to j1_repl   |  |
| args to j1_logger |  |


