
Implementation of the Strength Reduction optimization algorithm in Rust.  
Source files are in a toy SSA IR language (in `/examples/` directory).  

Implementation based on `Engineering a Compiler, second edtion` from `Keith Cooper, Linda Torczon`.  
Algorithm presented in section 10.7.2 page 580.

# Environment

Tested on `Ubuntu 18.04 LTS`,  with `rustc 1.44.0`

# Run

```
cargo build
cargo run <input-file>
```

# Test

```
cargo test
```
