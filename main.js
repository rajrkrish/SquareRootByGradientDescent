const iterations = 100000000;
const errorRate = 0.0005;

var i = 0;

var input = seed = 7588888654;

var solution = seed;
var error = Infinity;

if(input < 0){
    console.error("Cannot compute square roots for negative numbers");
    return;
}

while(i<iterations){
    error = solution**2 - input;
    if(error < errorRate && error > -errorRate){
        console.log("Solution Reached on iteration: ", i);
        break;
    }
    solution -= error/input;
    i++;
}


console.log('Solution: ', solution, 'Error: ', error);
