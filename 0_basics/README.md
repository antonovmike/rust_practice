Step 0: Become familiar with Rust basics
========================================

__Estimated time__: 3 days

Read through [Rust Book], [Rust FAQ], and become familiar with basic [Rust] concepts, syntax, memory model, type and module systems.

Polish your familiarity by completing [Rust By Example] and [rustlings].

Read through [Cargo Book] and become familiar with [Cargo] and its workspaces.

After completing these steps, you should be able to answer (and understand why) the following questions:

- What memory model [Rust] has? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?
Rust supports both single-threaded and multi-threaded programming, as well as synchronous and asynchronous code execution.

- What runtime [Rust] has? Does it use a GC (garbage collector)?
Rust has a minimal rantime, which consists of a small set of functions and types needed to run: panic handling, stack, allocator etc. Rust does not use GC.

- What statically typing means? What is a benefit of using it?
Static typing means that the types of variables, function parameters and return values are defined at compile time. It allows to detect errors related to data types in advance.

- What are generics and parametric polymorphism? Which problems do they solve?
Generics allow you to write generic algorithms that work with arbitrary types.
Parametric polymorphism is a language property that allows functions or data types to accept different types as parameters without depending on their specific values.
Generics and parametric polymorphism solve the problem of code duplication and rigid type-specific binding.

- What are traits? How are they used? How do they compare to interfaces? What are an auto trait and a blanket impl? What is a marker trait?
Traits define a set of methods that can be implemented for different data types. They allow describing certain behaviour.
Traits are used to create abstractions that can adapt to different situations, to implement polymorphism, to extend functionality, to define common patterns of behaviour.
A marker trait is a trait that does not contain any methods or associated elements, but only serves to mark types that have a certain property or support a certain protocol.
Auto trait is a special kind of trait that is automatically implemented for all types that satisfy certain conditions. For example, the Send trait is automatically implemented for all types that can be safely transferred between threads. Auto traits are used to define properties of types that do not depend on their behaviour, but on their internal structure.
Blanket impl is a trait implementation for all types that satisfy certain trait constraints. For example, the standard library has an empty implementation of the Iterator trait for all types that implement the IntoIterator trait. Empty implementations are used to provide common behaviour for a wide range of types that share some common characteristics.

- What are static and dynamic dispatches? Which should I use, and when?
Static dispatch is used by default when a specific type can be defined at compile time. Dynamic dispatching is used when the specific type that implements the trait is unknown at compile time.
Static dispatching is more efficient, but may increase the code size. Dynamic dispatching is less efficient, but is more flexible and can reduce code size.

- What is a crate and what is a module in Rust? How do they differ? How are the used?
Crate is a library or executable compiled from a package. 
Module is a container for elements. A module element is a module surrounded by curly braces, named and prefixed with the mod keyword. A module element introduces a new named module into the tree of modules that make up the crate. Modules can be nested arbitrarily

- What are move semantics? What are borrowing rules? What is the benefit of using them?
Move semantics is a way of programming in which values are passed by possession transfer. Once a value is moved from one variable to another, the first variable becomes invalid and cannot be used. Borrowing rules are compiler restrictions to ensure memory safety and prevent errors due to concurrent accesses or incorrect memory release. The advantage of using relocation and borrowing is that they reduce the cost of copying and rubbish collection, and ensure that values are not changed or destroyed unexpectedly

- What is immutability? What is the benefit of using it?
Immutability - in Rust, variables are immutable by default.
The advantage of using immutability is that it improves the reliability and readability of code, as well as facilitating parallel programming by eliminating the possibility of data and state races.

- What is cloning? What is copying? How do they compare?
Types that implement "Copy" can be copied via `memcpy`, for example when you pass an argument by value to a function.
I honestly never understood what "Clone" is, except that creating a new instance via "Clone" is more complicated than via "Copy".
"Clone" is explicit and "Copy" is not explicit.
For types that implement "Copy", the compiler understands that it does not need to treat them as a move.

- What is RAII? How is it implemented in [Rust]? What is the benefit of using it?
RAII (Resource Acquisition Is Initialisation) ensures that resources are released automatically when an object leaves scope. In Rust, RAII is implemented through destructors. When an object leaves scope, its destructor is called automatically, and any resources it owned will be released.

- What is an iterator? What is a collection? How do they differ? How are they used?
Collection is a container that stores a set of elements of the same type.
An Iterator is a mechanism for enumerating elements of a collection.
That is the difference between them. That's how they are used.

- What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?
Macros generate code based on templates. They are used to automate tasks so that you don't have to write repetitive code. In Rust, macros are of two types: declarative and procedural. Declarative macros are defined with `macro_rules!` and are simple conversions. Procedural macros are full-fledged Rust programs that manipulate token flow.


- How code is tested in [Rust]? Where should you put tests and why?
Unit tests are small and test a single module.
Integration tests are larger and test the interaction of modules with each other.
Tests should be placed in a separate tests directory in the root directory of the project. This makes it easy to find and run tests using the cargo test command.


- Why [Rust] has `&str` and `String` types? How do they differ? When should you use them?
&str is an immutable string that occupies a known amount of memory at compile time and cannot grow dynamically. &str is more memory efficient than String and is the preferred string type for text that is known at compile time and does not need to be modified.
String is a modifiable string type that can grow and change dynamically at runtime. String is used when a modifiable string is needed.


- What are lifetimes? Which problems do they solve? Which benefits do they give?
Lifetime is used to describe how long a reference remains valid based on the scope of the value it borrows. The Rust compiler uses "borrow check" to check the lifetime of a reference and ensure that it does not reference data that no longer exists. Lifetime annotations help resolve compiler errors and maintain memory safety when referencing. Lifetime only exists at compile time.
I suck at understanding Lifetime and have to google and go to the documentation every time.


- Is [Rust] OOP language? Is it possible to use SOLID/GRASP? Does it have an inheritance?
No. Rust has no inheritance and uses composition to create new objects by combining existing ones.

After you're done notify your lead in an appropriate PR (pull request), and he will exam what you have learned.

_Additional_ articles, which may help to understand the above topic better:
- [Chris Morgan: Rust ownership, the hard way][1]
- [Adolfo Ochagavía: You are holding it wrong][12]
- [Vikram Fugro: Beyond Pointers: How Rust outshines C++ with its Borrow Checker][15]
- [Sabrina Jewson: Why the “Null” Lifetime Does Not Exist][16]
- [HashRust: A guide to closures in Rust][13]
- [Ludwig Stecher: Rusts Module System Explained][2]
- [Tristan Hume: Models of Generics and Metaprogramming: Go, Rust, Swift, D and More][3]
- [Jeff Anderson: Generics Demystified Part 1][4]
- [Jeff Anderson: Generics Demystified Part 2][5]
- [Bradford Hovinen: Demystifying trait generics in Rust][14]
- [Brandon Smith: Three Kinds of Polymorphism in Rust][6]
- [Jeremy Steward: C++ & Rust: Generics and Specialization][7]
- [cooscoos: &stress about &Strings][8]
- [Jimmy Hartzell: RAII: Compile-Time Memory Management in C++ and Rust][9]
- [Georgios Antonopoulos: Rust vs Common C++ Bugs][10]
- [Yurii Shymon: True Observer Pattern with Unsubscribe mechanism using Rust][11]
- [Clayton Ramsey: I built a garbage collector for a language that doesn't need one][17]




[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://prev.rust-lang.org/faq.html
[rustlings]: https://rustlings.cool

[1]: https://chrismorgan.info/blog/rust-ownership-the-hard-way
[2]: https://aloso.github.io/2021/03/28/module-system.html
[3]: https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics
[4]: https://web.archive.org/web/20220525213911/http://jeffa.io/rust_guide_generics_demystified_part_1
[5]: https://web.archive.org/web/20220328114028/https://jeffa.io/rust_guide_generics_demystified_part_2
[6]: https://www.brandons.me/blog/polymorphism-in-rust
[7]: https://www.tangramvision.com/blog/c-rust-generics-and-specialization#substitution-ordering--failures
[8]: https://cooscoos.github.io/blog/stress-about-strings
[9]: https://www.thecodedmessage.com/posts/raii
[10]: https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust
[11]: https://web.archive.org/web/20230319015854/https://ybnesm.github.io/blah/articles/true-observer-pattern-rust
[12]: https://ochagavia.nl/blog/you-are-holding-it-wrong
[13]: https://hashrust.com/blog/a-guide-to-closures-in-rust
[14]: https://gruebelinchen.wordpress.com/2023/06/06/demystifying-trait-generics-in-rust
[15]: https://dev.to/vikram2784/beyond-pointers-how-rust-outshines-c-with-its-borrow-checker-1mad
[16]: https://sabrinajewson.org/blog/null-lifetime
[17]: https://claytonwramsey.github.io/2023/08/14/dumpster.html
