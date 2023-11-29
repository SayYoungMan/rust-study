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

## 10.2. Traits: Defining Shared Behavior

- A `trait` defines functionality a particular type has and can share with other types.
- We can use traits to define shared behavior in an abstract way. We can use `trait bounds` to specify that a generic type can be any type that has certain behavior.
- Traits are similar to a feature often called `interfaces` in other languages.

### Defining a Trait

- Different types share the same behavior if we can call the same methods on all of those types.
- Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

### Implementing a Trait on a Type

- Implementing a trait on a type is similar to implementing regular methods but after `impl`, we must put the trait name we want to implement, use `for` keyword and specify the name of type we want to implement for.
- Note, we can implement a trait on a type only if at least one of the trait or the type is local to our crate.
- This restriction is part of a property called `coherence`, and more specifically the `orphan rule`, so named because the parent is not present. This rule ensures that other people's code can't break your code and vice versa. Without this, two crates could implement the same trait for the same type.

### Default Implementations

- It's useful to have a default behaviour on some methods in a trait and let each type override the default behavior if they need to.
- Default implementations can call other methods in the same trait, even if those other methods don't have a default implementation.

### Traits as Parameters

- To define functions that accept many different types but same Trait, we specify the `impl` keyword and the trait name. This parameter accepts any type that implements the specified trait.

#### Trait Bound Syntax

- The `impl Trait` syntax is actually a syntax sugar for a longer form known as `trait bound`:
  ```rust
  pub fn notify<T: Summary>(item: &T) {}
  ```
- If you want to force parameters to have the same type, we must use a trait bound:
  ```rust
  pub fn notify<T: Summary>(item1: &T, item2: &T) {}
  ```

#### Specifying Multiple Trait Bounds with the + Syntax

- We can specify more than one trait bound to use types that satisfies multiple traits:
  ```rust
  pub fn notify(item &(impl Summary + Display)) {}
  pub fn notify<T: Summary + Display>(item &T) {}
  ```

#### Clearer Trait Bounds with where Clauses

- Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature to make it less cluttered:
  ```rust
  fn some_function<T, U>(t: &T, u: &U) -> i32
  where
      T: Display + Clone,
      U: Clone + Debug,
  ```

### Returning Types that Implement Traits

- We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait.
- This is useful in the context of closures and iterators which create types that only the compiler knows or types that are very long to specify.
- However, you can only use `impl Trait` if you're returning a single type.

### Using Trait Bounds to Conditionally Implement Methods

- By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
- We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on anytype that satisfies the trait bounds are called `blanket implementations`.
  ```rust
  impl <T: Display> ToString for T
  ```
