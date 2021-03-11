# Caravel Compiler

This repository hosts both the frontend (lexer, parser) and the caravel-to-javascript compiler backend.

# Setup

The Caravel compiler relies on experiment features. The project must be built using the nightly toolchain toolchain.

1. Install nightly toolchain

```
$ rustup toolchain install nightly
```

2. Set nightly as default

```
$ rustup override set nightly
```

3. Run!

```
$ cargo run
```