cargo run -- input.cv && \
clang stdlib.c -c -o stdlib.o &&  \
clang out.o stdlib.o -o program && \
./program