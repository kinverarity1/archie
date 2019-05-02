use std::io;


fn main() {
    println!("Archie's law: porosity = (a / F) ^ (1 / m)");
    println!("Formation factor F = Ro / Rw");

    println!("Ro? ");
    let mut ro = String::new();
    io::stdin()
        .read_line(&mut ro)
        .expect("Please enter something");
    let ro: f64 = ro.trim().parse().expect("Please enter a number");

    println!("Rw? ");
    let mut rw = String::new();
    io::stdin().read_line(&mut rw).expect("Please enter something");
    let rw: f64 = rw.trim().parse().expect("Please enter a number");

    println!("Formation factor = {}", ro / rw);
}
