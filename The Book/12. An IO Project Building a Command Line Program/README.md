# 12. An I/O Project: Building a Command Line Program

- We'll build a command line tool that interacts with file and command line input/output to practice some of the Rust concepts.
- We are making a `grep` that searches a specified file for a specified string.
- To do so, `grep` takes arguments a file path and a string, then it reads the file, finds lines that contain the string and prints those lines.
- We'll read environment variables to allow the user to configure the behavior of our tool.
- We'll also print error messages to the standard error console stream instead of standard output so the user can redirect successful output to a file while still seeing error messages onscreen.

## 12.1. Accepting Command Line Arguments

- `std::env::args` will panic if any argument contains invalid Unicode. If you need it to accept invalid Unicode, use `std::env::args_os` instead.
