# ScriptS
this programming language is design for alternative of Python.
Because Python function is very rich, but that grammar is too bad.

ScriptS transpile from awesome ScriptS soruce code to Python code.
Python code will run on background.
[Sila](https://github.com/KajizukaTaichi/sila) is used as transpiler infrastructure.

In meaning, S stands for Speed, Secure, Simple and Small.

## [Example code](/example.ss)
```
// This program is to solve FizzBuzz;

fn fizzbuzz(i) {
    if i % 15 == 0 {
        return "FizzBuzz";
    };
    if i % 5 == 0 {
        return "Buzz";
    };
    if i % 3 == 0 {
        return "Fizz";
    };
    return str(i);
};

fn add_text(source, text) {
    return source + text + " "
};

var i <- 0;
var result <- "";
while i < 100 {
    var i <- i + 1;
    var result <- add_text(result, fizzbuzz(i));
};
print result;
```

## Usage
Rust is needed to use ScriptS. You have to install it.

sub-command `code` is to run script.
```
cargo run -- run example.ss
```

sub-command `code` is to show Python code generated.
```
cargo run -- code example.ss
```

sub-command `ast` is to show abstract syntax tree.
```
cargo run -- ast example.ss
```
