# IITK - Traveller
Source code of the CLI version of the IITK-Traveller Esolang, built by Programming Club IIT Kanpur. The documentation of the working of the language can be found at [https://iitk-traveller-docs.netlify.app/](https://iitk-traveller-docs.netlify.app/)

# Dependencies
Working installation of the Rust Programming Language and the cargo manager is required to build the project. The project was developed in **rustc v1.65** and **cargo v1.65**. Install by following the [official installation](https://doc.rust-lang.org/cargo/getting-started/installation.html#:~:text=On%20Windows%2C%20download%20and%20run,channels%20for%20Rust%20and%20Cargo.) guide according to your platform.

# Building the Project and executing code
Clone this repo to working directory and open a new terminal (powershell for windows) on the same directory. Then
```bash
cd iitk_traveller
cargo run <path to your program>   #builds the executable in /target/debug and runs the program at path given
```

### transpiler.cpp
As a fun aside, we developed a transpiler that converts any code written in [brainfuck](https://esolangs.org/wiki/Brainfuck) to it's IITK-Traveller equivalent, proving in the process that it is infact Turing complete. To run the transpiler, simply compile the cpp file and,
```bash
./transpiler <path to brainfuck code>
```
The equivalent IITK_Traveller code will be dumped to your terminal.