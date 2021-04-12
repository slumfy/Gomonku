pub fn print_axes(axes: &[[u16; 4]; 2]) {
    println!("white axes : ");
    println!();
    println!("|----------------|");
    for axe in &axes[0] {
        print!("|");
        print!("{:016b}", axe);
        print!("|");
        println!();
    }
    println!("|----------------|");
    println!();
    println!("black axes : ");
    println!();
    println!("|----------------|");
    for axe in &axes[1] {
        print!("|");
        print!("{:016b}", axe);
        print!("|");
        println!();
    }
    println!("|----------------|");
}
