# Provola

> Provola, provalo, just test it!

`provola` is your best ~~cheese~~ friend when you need to test code quickly.

Just create an *input* file, an *expected output* file and a program digest the
input and `provola` will test your program automatically whenever
you change the code.

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
