use std::io;

fn fib(n: u32) -> u64 {
    let mut pair = (0, 1);
    for _ in 0..n {
        let (a, b) = pair;
        pair = (b, a + b);
    }
    pair.0
}

fn main() {
    let mut read_n: bool = true;
    while read_n {
        println!("Enter n value:");
        let mut n = String::new();
        io::stdin().read_line(&mut n)
            .expect("Failed to read line");
    
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        read_n = false;
        
        let ans: u64 = fib(n);
        
        println!("Fib({}) : {}", n, ans);
    }
}
