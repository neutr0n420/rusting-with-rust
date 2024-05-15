# Ownership in Rust

## Stack vs Heap

Rust has clear rules about stack and heap data managment:

- Stack: Fact allocation and deallocation. Rust uses stack for most primitive data types and for data where the size is known at compile time
- Heap: Used for data that can grow at runtime such as vectors or string

## Ownership rules

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be droped

## References

1. A reference is a kind of pointer.
2. Reference mean giving the address of a string rather than the ownership of the string over to function
