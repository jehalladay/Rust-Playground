use std::env;


fn processor() {
    println!("process");
    return ();
}


fn unittest() {
    println!("hello");
    return ();
}





fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "test" {
        unittest();
        println!("{}", args[1]);
    } else {
        processor();
        println!("{}", args[1]);
    }
}