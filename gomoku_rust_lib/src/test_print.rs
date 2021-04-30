use crate::print::print_pos_in_human_format;

pub fn test_print_pos_in_human_format() {
    println!("printing pos 0");
    print_pos_in_human_format(0);
    println!("printing pos 180");
    print_pos_in_human_format(180);
    println!("printing pos 360");
    print_pos_in_human_format(360);
}
