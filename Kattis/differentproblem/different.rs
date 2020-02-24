use std::io;

// use std::io::BufRead;


// fn main() {
//     let stdin = io::stdin();
//     for line in stdin.lock().lines().map(|l| l.unwrap()) {
//         let nums: Vec<i64> = line.split(" ")
//             .map(|num| num.parse().unwrap())
//             .collect();
//         let a = nums[0];
//         let b = nums[1];
//         println!("{}", (a - b).abs());
//     }
// }



fn main() {
    let x: bool = true;
    while x {
        let mut y = String::new();
        io::stdin().read_line(&mut y)
            .expect("failed to read line");
        let split = y.split(" ");
        let vec: Vec<&str> = split.collect();
        let vec2: Vec<&str> = vec.map(|a| a.parse());
    
        println!("{}", vec[0]);
    }



}