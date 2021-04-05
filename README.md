# Uintarray
Stores a number of uints in a single uint. Their collective size in bits cannot exceed that of the uint that stores them.

Why does this exist? Because I like implementing it in various languages, and now came rusts turn.

## Usage
The api closely matches that of a Python list, with the limitation that it can't grow above a certain size, decided by the size of its elements.

Using a uintarray is straight forward, just include the namespace and specify a size.
```rust
use uintarray::UintArray;
let ua = UintArray::new::<u8>();
```
We can now start adding elements to `ua`
```rust
let ua = ua
    .append(0)
    .append(1)
    .append(2);
```
Note that we reassign it afterwards. Each call to `append` returns a new UintArray with the element added at the end. This is also true for any other method that would mutate the UintArray, because the UintArray is actually immutable.

The UintArray also implements the trait `IntoIter`, meaning that we can iterate the elements using a for loop.
```rust
for elem in ua {
    print!("{} ", elem);
}

// prints:
// 0 1 2
```
For a more elaborate example, check [main.rs](src/main.rs)

## Docs

Docs are available using `cargo doc --open`
