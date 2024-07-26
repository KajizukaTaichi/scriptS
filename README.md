# ScriptS
this programming language is design for alternative of Python.
Because Python function is very rich, but that grammar is too bad.

ScriptS transpile from awesome ScriptS soruce code to Python code.
[Sila](https://github.com/KajizukaTaichi/sila) is used as transpiler infrastructure.

In meaning, S stands for Speed, Secure, Static, Simple and Small.
That doesn't include Soviet, Stalin and Socialism. lol

## [Example code](/example.ss)
```
print "Multiplication table";
var i <- 0;
while i < 10 {
    var i <- i + 1;
    var line <- "";

    var j <- 0;
    while j < 10 {
        var j <- j + 1;
        var line <- line + " " + str(i * j).rjust(3);
    };
    print line;
};
```

## Usage
Rust is needed to use ScriptS. You have to install it.
Let's try below command to transpile Python code.
```
cargo run -- example.ss example.py
```
Then, It may created file `example.py`.
After that, just run it as Python.
