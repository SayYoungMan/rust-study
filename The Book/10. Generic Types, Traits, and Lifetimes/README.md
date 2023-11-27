# 10. Generic Types, Traits, and Lifetimes

- `Generics` is an abstract stand-ins for concrete types or other properties.
- We can express the behaviour of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.
- You can combine `traits` with generic types to constrain a generic type to accept only those typ4es that have a particular behaviour.
- `Lifetimes` is a variety of generics that give the compiler information about how references relate to each other.
- Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure references will be valid in more situations.

## 10.1. Generic Data Types

### In Function Definitions

- We place the generics in the signature of the function.
- To You parameterize the types in a function by defining type parameters and we use `T` by convention.

### In Struct Definitions

- The syntax for using generics in struct definitions is similar to that used in function definitions.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

- If you try to create an instance that has values of different types in the above example, it won't compile because we are using same type T.
- To make it possible to have to different values, define two separate type parameters and assign them.

### In Enum Definitions

- We can define enums to hold generic data types in their variants like `Option<T>`.

### In Method Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

- We have to declare T just after `impl` so we can use T to specify that we're implementing methods on the type `Point<T>`.
- Method written this way will be defined on any instance of the type, no matter what concrete type of the instance.
- We can also define method only on instances of certain type.
  ```rust
  impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
  }
  ```
-

### Performance of Code Using Generics

- Using generic types won't make your program run any slower than it would with concrete types.
- This is accomplished by `monomorphization`, which is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
- The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.
