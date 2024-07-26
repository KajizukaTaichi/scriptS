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
