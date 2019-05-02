use std::io;


fn main() {
    println!("Archie's law: porosity = (a / F) ^ (1 / m)");
    println!("Formation factor F = Ro / Rw");

    let ro = get_param("Ro");
    let rw = get_param("Rw");
    println!("Formation factor = {}", ro / rw);
    let a = get_param("a");
    let m = get_param("m");

    let porosity = (a / (ro / rw)).powf(1. / m);
    println!("-> porosity {}", porosity * 100.);
}

fn get_param(label: &str) -> f64 {
    let mut line = String::new();
    println!("{}?", label);
    io::stdin().read_line(&mut line).expect("Please enter a value");
    line.trim().parse().expect("Please enter a floating point number")
}