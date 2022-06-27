# applesimd

This crate provides Rust types with the same memory layout as types in Apple's [simd](https://developer.apple.com/documentation/accelerate/simd) library.  
Those types are widely used with [Accelerate](https://developer.apple.com/documentation/accelerate) or [Metal](https://developer.apple.com/metal/),
which often expect programmers to express their values in those layouts.

## Design notes
I have deliberately chosen not to implement or bind most of the functions in the [simd](https://developer.apple.com/documentation/accelerate/simd) library, which consists largely in apple's header
definitions, and more occasionally in some "semi-private" (that is, ABI-stable) symbol.

This is because I benchmarked it, and doing the obvious thing with Rust using `#[repr(Rust)]` memory layout is typically
faster.  However your mileage may vary, and to the extent I uncover performance improvements that are practical I may revisit this.

Consequently, I believe this library is mostly useful for cases where you want to integrate with some other code or 
environment which insists on Apple-style memory layout, rather than being a natural solution for most Rust programs.

# Implementation status
The following types are implemented:

* Int3
* Int4
* Float2
* Float3
* Float4
* Float3x3