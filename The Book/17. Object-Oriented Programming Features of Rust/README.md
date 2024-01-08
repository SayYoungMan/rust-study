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

## 17.2. Using Trait Objects that Allow for Values of Different Types

- Sometimes we want our library user to be able to extend the set of types that are valid in a particular situation.
- To show how to achieve this, we will create an example GUI that iterates through a list of items, calling a `draw` method on each one to draw it to the screen.
- In a language with inheritance, we might define a class named `Component` that has a method named `draw` on it. The other classes, such as `Button`, would inherit from `Component`.

### Defining a Trait for Common Behavior

- A `trait object` points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime.
- We create a trait object by specifying some sort of pointer, such as & or `Box<T>`, then `dyn` keyword, then specifying the relevant trait.
- We can use trait objects in place of a generic or concrete type. Wherever we use a trait object, Rust's type system will ensure at compile time that any value used in that context will implement the trait objec's trait.
- Trait objects are more like objects in other languages in the sense that they combine data and brhavior. But trait objects differ from traditional objects in that we can't add data to a trait object.
- If you'll only ever have homogeneous collections, using generics and trait bounds is preferable because the definitions will be monomorphized at compile time to use the concrete types.

### Implementing the Trait

- The concept of being concerned only with the messages a value responds to rather than the value's concrete type is similar to the concept of `duck typing` in dynamically typed languages.
- The advantage of using trait objects and Rust's type system to write code similar to code using duck typing is that we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn't implement a method but we call it anyway.

### Trait Objects Perform Dynamic Dispatch

- Recall that compiler performs monomorphization when we use trait bounds on generics: the compiler generates nongeneric implementations of functions and methods for each concrete type.
- The code that results is doing `static dispatch`, meaning that when the compiler knows what method you're calling at compile time.
- This is opposed to `dynamic dispatch`, which is when the compiler can't tell at compile time which method you're calling. In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
- When using trait objects, Rust must use dynamic dispatch because the compiler doesn't know all the types that might be used with the code.
- Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. This lookup incurs a runtime cost that doesn't occur with static dispatch.
- Dynamic dispatch also prevents the compiler from choosing to inline a method's code, which in turn prevents some optimizations.
