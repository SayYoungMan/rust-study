# 17. Object-Oriented Programming Features of Rust

## 17.1. Characteristics of Object-Oriented Languages

### Objects Contain Data and Behavior

- Object-oriented programs are made of objects.
- An `object` packages both data and the procedures that operate on that data.
- The procedures are typically called `methods` or `operations`.
- Using this definition, Rust is object-oriented: structs and enums have data, and `impl` blocks provide methods on structs and enums.

### Encapsulation that Hides Implementation Details

- `Encapsulation` means that the implementation details of an object aren't accessible to code using that object.
- Only way to interact with an object is through its public API; code using the object shouldn't be able to reach into the object's internals and change data or behavior directly.
- This enables the programmers to change an object's internals without needing to change the code that uses the object.
- In Rust, the option to use `pub` or not for different parts of code enables encapsulation of implementation details.

### Inheritance as a Type System and as Code Sharing

- `Inheritance` is a mechanism whereby an object can inherit elements from another object's definition, thus gaining the parent object's data and behaviour without you having to define them again.
- In Rust, there is no way to define a struct that inherits the parent struct's fields and method implementations without using a macro.
- You would choose inheritance for two reasons:
  1. For reuse of code
     - This can be done in a limited way in Rust using default trait method implementations.
  2. To enable child type to be used in the same places as the parent type. This is also called `polymorphism`, which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.
     - Rust uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. (`bounded parametric polymorphism`)
- Inheritance is less favoured recently because subclasses shouldn't always share all characteristics of parent class as it makes program's design less flexible. This also introduces possibility of calling methods on subclasses that don't make sense.
