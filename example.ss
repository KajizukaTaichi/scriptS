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
