use std::io;


fn main() {
    println!("Archie's law: porosity = (a / F) ^ (1 / m)");
    println!("Formation factor F = Ro / Rw");

    let ro = get_param("Ro".to_string());
    let rw = get_param("Rw".to_string());
    println!("Formation factor = {}", ro / rw);
    let a = get_param("a".to_string());
    let m = get_param("m".to_string());

    let porosity = (a / (ro / rw)).powf(1. / m);
    println!("-> porosity {}", porosity * 100.);
}

fn get_param(label: String) -> f64 {
    let mut line = String::new();
    println!("{}?", label);
    io::stdin().read_line(&mut line).expect("Please enter a value");
    line.trim().parse().expect("Please enter a floating point number")
}