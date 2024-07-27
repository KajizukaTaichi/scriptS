// This function is to solve FizzBuzz;
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

var i <- 0;
while i < 100 {
    var i <- i + 1;
    print fizzbuzz(i);
};
