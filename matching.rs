fn main () {
    let x = (21, true);
    match x {
        (h, k) if h > 20 && h < 26 && k == true => { println("a"); }
        (_, k) if k == true => { println("b"); }
        (h, _) if h > 40 && h < 49 => { println("c"); }
        (_, _) => { println("d"); }
    }
}
