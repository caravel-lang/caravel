# Build Caravel code
cargo run -- input.cv

# Build stdlib
clang stdlib.c -c -o stdlib.o

# Link
clang out.o stdlib.o -o program

# Run
./program