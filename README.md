# What is it ?

Izber is a compiler which generates assembly code from high level code

# Iz.. what ?

Izber, short for Izberbash, a city in the South of Russia

# Example of the source program (Now Izber can compile only declarations)

```
def main() {
    let a: uint32;
    let b: uint64;
}
```

# How to use Izber
Firstly you have to build it:

```bash
cargo build
```

Secondly create source file called `input.iz` and insert code from the example

Compiling source:
```bash
./izber input.iz
```

Assembling Izber's output:
```bash
gcc input.s -o input
```

Run the generated (by assembler) file:
```bash
./input
```
