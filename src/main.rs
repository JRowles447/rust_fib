use std::io;

fn main() {
    println!("Enter value of n for which to compute the nth fibonacci number!"); 

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input, please provide a number"); 
    
    let n: i64 = match n.trim().parse() {
        Ok(num) => {
            if num < 0 {
                println!("Cannot compute fib of negative number!");
                0
            } else {num}
        },
        Err(_) => 0
    };

    println!("Computing fib({})\n", n);

    let res = fib(n);

    println!("\nfib({})={}", n, res);

}

// start with fib(1)=0 
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34
fn fib(n: i64) -> i64 {
    let mut arr = [0, 1];

    let mut ind = 1; 
    
    // build up the value using array
    // pull this out to a fib() function 
    while ind < n {
        ind += 1;
        print!("{}, ", arr[0]);

        let replace = arr[1];

        arr[1] = arr[0] + arr[1];
        arr[0] = replace;
    }

    print!("{}", arr[0]);

    arr[0]
}