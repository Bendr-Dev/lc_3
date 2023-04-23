# lc_3
An LC-3 virtual machine written in Rust for learning purposes

Utilized some great documentation and tutorials for the LC-3:

[LC3 ISA PDF](https://www.cs.colostate.edu/~cs270/.Spring21/resources/PattPatelAppA.pdf)

[Tutorial written in C](https://www.jmeiners.com/lc3-vm/)

# Usage

1. Clone/fork the repo
2. Make sure you have `rustc` installed (at the time this was built version `1.69.0`)
3. Go into project's top level directory inside a terminal
4. In the CLI run `cargo run -- resources/[file-name].obj` ex: `cargo run -- resources/2048.obj`
    * Note: If you are not on a Unix-based OS, you will not be able to run due to differences in system calls
    
    However, if you are on a Windows machine, opening a remote connection to a WSL hosting a Unix-based OS will allow you to compile and run this virtual machine
    
    [Installing Ubuntu on WSL2](https://ubuntu.com/tutorials/install-ubuntu-on-wsl2-on-windows-11-with-gui-support#2-install-wsl)
    
    [Installing WSL/WSL extension on VSCode](https://code.visualstudio.com/docs/remote/wsl)
