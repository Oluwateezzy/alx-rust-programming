# Rust Programming

A collection of Rust programming examples and exercises focusing on fundamental concepts and variable handling.

## Project Structure

```
alx-rust-programming/
├── basics/
│   ├── allow.rs           # Handling unused variables
│   ├── destructuring.rs   # Tuple and array destructuring
│   ├── let.rs            # Basic variable binding with let
│   ├── mut.rs            # Mutable variables
│   ├── scope.rs          # Variable scope concepts
│   ├── scope2.rs         # Advanced scope examples
│   ├── shadowing.rs      # Variable shadowing concepts
│   └── shadowing2.rs     # Advanced shadowing and rebinding
└── README.md
```

## Learning Topics Covered

### 1. Variable Binding (`let.rs`)
Basic variable declaration and binding in Rust:
- Immutable variable binding with `let`
- Type annotations
- Variable initialization

### 2. Mutability (`mut.rs`)
Understanding mutable variables:
- Using `mut` keyword for mutable variables
- Modifying variable values
- The difference between immutable and mutable bindings

### 3. Variable Shadowing (`shadowing.rs`, `shadowing2.rs`)
Exploring Rust's shadowing feature:
- Redefining variables with the same name
- Shadowing vs. mutability
- Type changing through shadowing
- Scope-based shadowing

### 4. Variable Scope (`scope.rs`, `scope2.rs`)
Understanding variable scope in Rust:
- Block scope with curly braces `{}`
- Variable lifetime and accessibility
- Function scope
- Nested scope behavior

### 5. Destructuring (`destructuring.rs`)
Pattern matching and destructuring:
- Tuple destructuring with `let (x, y) = (1, 2)`
- Array destructuring
- Partial destructuring with `..` syntax
- Destructuring assignments

### 6. Compiler Attributes (`allow.rs`)
Working with compiler warnings:
- Handling unused variable warnings
- Understanding Rust's strict compiler checks

## How to Run the Examples

Each Rust file can be compiled and run individually:

```bash
# Compile and run a specific example
rustc basics/let.rs -o let_example && ./let_example

# Or use cargo run (if you have a Cargo.toml file)
# cargo run --bin example_name
```

### Running All Examples

You can run all examples in the basics folder:

```bash
cd basics
for file in *.rs; do
    echo "Running $file..."
    rustc "$file" -o "${file%.rs}" && "./${file%.rs}"
    rm "${file%.rs}"  # Clean up binary
    echo "---"
done
```

## Key Rust Concepts Demonstrated

1. **Immutability by Default**: Variables are immutable unless explicitly marked with `mut`
2. **Type Safety**: Rust's strict type system prevents many common programming errors
3. **Ownership and Borrowing**: Basic concepts around variable ownership
4. **Pattern Matching**: Using destructuring to extract values from complex data types
5. **Scope Management**: Understanding when variables are valid and accessible

## Prerequisites

- Rust compiler (`rustc`) installed
- Basic understanding of programming concepts
- Familiarity with command line interface

## Installation

Make sure you have Rust installed on your system:

```bash
# Install Rust via rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
```

## Learning Path

It's recommended to go through the examples in this order:

1. `let.rs` - Basic variable binding
2. `mut.rs` - Understanding mutability
3. `scope.rs` - Basic scope concepts
4. `scope2.rs` - Advanced scope examples
5. `shadowing.rs` - Variable shadowing
6. `shadowing2.rs` - Advanced shadowing and rebinding
7. `destructuring.rs` - Pattern matching and destructuring
8. `allow.rs` - Compiler attributes and warnings

## Common Rust Patterns Shown

- **Variable Declaration**: `let x = 5;`
- **Mutable Variables**: `let mut x = 5;`
- **Type Annotations**: `let x: i32 = 5;`
- **Tuple Destructuring**: `let (x, y) = (1, 2);`
- **Array Destructuring**: `[.., y] = [6, 3];`
- **Assertions**: `assert_eq!(x, 5);`
- **Print Macros**: `println!("{}", x);`

## Notes

- All examples include practical demonstrations of core Rust concepts
- Some files contain commented-out code showing common mistakes or alternative approaches
- The examples progress from basic to more advanced concepts
- Each file focuses on a specific aspect of Rust variable handling

## Contributing

1. Try to understand each concept before moving to the next
2. Experiment with the code by making small modifications
3. Pay attention to compiler error messages - they're very helpful in Rust
4. Practice writing similar code from scratch

## Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

---
