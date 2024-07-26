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
