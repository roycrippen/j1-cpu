# J1 CPU emulator

[J1](https://excamera.com/sphinx/fpga-j1.html) CPU emulator written in Rust. <br>
Ported from [j1](https://github.com/dim13/j1) written in go. <br>
requires [rustup](https://rustup.rs/) <br>

### build and install
```shell
$ cargo install --path . 
```
### executables
Help with executable arguments `<executable> -h` or `<executable> --help` <br>

| Name                        |Description |
| :-------------------------  | :------ |
| j1                          | j1 emulator |
| j1_dump                     | dump j1 cpu memory in assembly or instruction AST format |
| j1_example_compile_and_dump | example of programmatically using j1 |

### test
```shell
$ cargo test
```

### document
```shell
$ cargo doc
# open j1-cpu/target/doc/j1/index.html with browser
```

### run j1 eforth repl
```shell
# option -r or --repl
$ j1 --repl
```

### run j1 eforth repl with a script
```shell
# from j1-cpu directory
$ cd resources
$ j1 --repl --script simple.fth
```

### j1 options
```shell
# help -h or --help
# Note: results saved to <script_file>-log.txt if not running repl
$ j1 -h

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
| Task                       | Done |
| :------------------------  | :------: |
| stack                      | &#x2714; |
| instructions               | &#x2714; |
| cpu                        | &#x2714; |
| dump memory to asm and ast | &#x2714; |
| j1 eforth emulator         | &#x2714; |
| args fo j1                 | &#x2714; |
| j1 gRPC service            |  |
| port to j1-swap            |  |
| add verbosity levels       |  |
