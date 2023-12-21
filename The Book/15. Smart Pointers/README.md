# 15. Smart Pointers

- `References` have no special capabilities other than referring to data and have no overhead.
- `Smart pointers` are data structures that act like a pointer but also have additional metadata and capabilities.
- `Reference counting` smart pointer type enables you to allow data to have multiple owners by keeping track of owners and when no owners remain, clean up the data.
- While references only borrow data, in many cases, smart pointers own the data they point to.
- `String` is an example of smart pointer that own some memory and allow you to manipulate it. Also have capacity as metadata and has extra ability to ensure the data will always be valid UTF-8.
- Smart pointers are usually implemented using structs. Unlike ordinary struct, they implement `Deref` and `Drop` traits.
  - `Deref` trait allows an instance of smart pointer struct to behave like a reference.
  - `Drop` trait allows you to customize the code that's run when an instance of the smart pointer goes out of scope.
