# Provola

> Provola, provalo, just test it!

`provola` is your best ~~cheese~~ friend when you need to test code quickly.

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

## Shell auto completion

`provola` provides shell autocompletion for even faster usage.

You can install autocompletion for yout favourite shell:

```shell
provola --generate zsh > ~/.zfunc/_provola
```

## Supported languages

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

`provola` is able to use test runners generated by common test frameworks.

Example:

```shell
provola -T GoogleTest -t path/to/gtest/executable
```

This is a (work in progress) list of supported frameworks:

| Framework   | Language |
|-------------|----------|
| Google Test | C++      |
