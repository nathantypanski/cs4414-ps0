use std::os;

fn main() {
    if os::args().len() < 2 {
        println("No arguments provided; running test cases ...");
        show_steps(6);
        show_steps(45);
        show_steps(260);
        return;
    }
    let arg = from_str::<int>(os::args()[1]).unwrap();
    show_steps(arg);
}

fn show_steps(x : int) {
    print!("{:d} steps -> collatz({:d})\n", steps(x), x);
}

fn steps(x: int) -> int {
    let mut j = 1;
    while collatz(j) != x {
        j = j + 1;
    }
    j
}

fn collatz(N: int) -> int {
    if N == 1 { return 0; }
    match N % 2 {
        0 => { 1 + collatz(N/2) }
        _ => { 1 + collatz(N*3+1) }
    }
}
