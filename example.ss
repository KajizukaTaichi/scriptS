var i <- 0;
while i < 100 {
    var i <- i + 1;
    if i % 15 == 0 {
        print "FizzBuzz";
    }; if !(i % 15 == 0) {
        if i % 5 == 0 {
            print "Buzz";
        }; if !(i % 5 == 0) {
            if i % 3 == 0 {
                print "Fizz";
            }; if !(i % 3 == 0) {
                print i
            };
        };
    };
};
