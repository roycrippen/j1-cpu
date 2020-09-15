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
./target/release/j1 --repl
```

### j1 forth options
```shell
# help -h or --help
# Note: results saved to <script_file>-log.txt if not running repl

./target/release/j1 -h

j1 1.0
Roy Crippen
J1 cpu emulator

USAGE:
    j1 [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -r, --repl       Run the J1 forth repl
    -V, --version    Prints version information

OPTIONS:
    -b, --bin <bin_file>          Binary J1 forth imamge to load
    -s, --script <script_file>    Forth script file to load and execute
```


### todo
| Task                  | Done |
| :-------------------  | :------: |
| stack                 | &#x2714; |
| instructions          | &#x2714; |
| cpu                   | &#x2714; |
| dump bin to asm       |  |
| j1 forth emulator     | &#x2714; |
| args fo j1            | &#x2714; |
| j1 gRPC service       |  |
| port to j1-hacked     | |
| add verbosity levels  | |
