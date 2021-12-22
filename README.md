# Provola

> ~~Test it!~~ → ~~Provalo!~~ → Provola!

![Provola Logo](/.doc/provola-icon.png)

`provola` is your best ~~cheese~~ friend when you need to test code quickly.

## Input/Output test

Just create an *input* file, an *expected output* file and a program to digest
the input and `provola` will test your program automatically whenever you change
the code.

```shell
# Create a simple haskell program
echo 'main = interact reverse' > reverse.hs

# An input for your nice program
echo -n abcd > in.txt

# The expected output
echo -n dcba > out.txt

# Run provola!
provola -i in.txt -o out.txt -s reverse.hs -w .
```

Where:

- `-i` is the input file
- `-o` is the expected output file
- `-s` is the source code
- `-w` is the file or directory to watch for changes

### Supported languages

| Language   | Build       | Run      |
|------------|-------------|----------|
| Bash       | /           | ✓        |
| C++        | `gcc`       | ✓        |
| C          | `gcc`       | ✓        |
| Haskell    | `stack ghc` | ✓        |
| JavaScript |             | ✓ `node` |
| PHP        |             | ✓        |
| Python     |             | ✓        |
| Rust       | `rustc`     | ✓        |

## Test frameworks

`provola` is able to use test runners generated by popular test frameworks.

Example:

```shell
provola -T GoogleTest -t path/to/gtest/executable
```

![GoogleTest example](/.doc/googletest-screenshot.png)

Where:

- `-T` is the test runner type (e.g.: GoogleTest, Catch2, ...)
- `-t` is the executable to run

You don't need to specify what files to watch, because in this case `provola` is
automatically watching changes in the test runner (not the sources!).

You can also find an example of GoogleTest runner inside
`provola-googletest/examples/data/`

This is a (work in progress) list of supported frameworks:

| Framework   | Language |
|-------------|----------|
| Google Test | C++      |
| Catch2      | C++      |

## Install

If you have a rust toolchain, just clone this project and from the root
directory:

```shell
cargo install --path .
```

If you want to use the GUI, you need to enable the egui feature:

```shell
cargo install --features=egui --path .
```


## Shell auto completion

`provola` provides shell autocompletion for even faster usage.

You can install autocompletion for yout favourite shell:

```shell
provola --shell-compl zsh > ~/.zfunc/_provola
```
