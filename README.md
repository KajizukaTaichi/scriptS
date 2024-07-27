# ScriptS
this programming language is design for alternative of Python.
Because Python function is very rich, but that grammar is too bad.

ScriptS transpile from awesome ScriptS soruce code to Python code.
Python code will run on background.
[Sila](https://github.com/KajizukaTaichi/sila) is used as transpiler infrastructure.

In meaning, S stands for Speed, Secure, Simple and Small.

## [Example code](/example.ss)
```
// This program to solve FizzBuzz;
var i <- 0;
var result <- "";
while i < 100 {
    var i <- i + 1;
    if i % 15 == 0 {
        var result <- result + "FizzBuzz ";
    }; if !(i % 15 == 0) {
        if i % 5 == 0 {
            var result <- result + "Buzz ";
        }; if !(i % 5 == 0) {
            if i % 3 == 0 {
                var result <- result + "Fizz ";
            }; if !(i % 3 == 0) {
                var result <- result + f"{i} ";
            };
        };
    };
};
print result;
```

## Usage
Rust is needed to use ScriptS. You have to install it.
Let's try below command to run script.
```
cargo run -- example.ss
```
