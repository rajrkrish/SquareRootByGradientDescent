fn main(){
    const ITERATIONS:i32 = 100000000;
    const ERROR_RATE:f64 = 0.0005;

    let input:f64 = 7588888654.0;

    let seed = input;
    let mut solution = seed;
    let mut error:f64 = -1.0; 

    if input < 0.0 {
        println!("Cannot compute square roots for negative numbers");
        return;
    }

    for i in 0..ITERATIONS {
        error = f64::powi(solution,2) - input;
        if error < ERROR_RATE && error > -ERROR_RATE {
            println!("Solution Reached on iteration: {0}", i);
            break;
        }
        solution -= error/input;
    }

    println!("Solution: {0}\tError: {1}", solution, error);
}