fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print_modulo(a: i32, b: i32) {
    println!("{}", a % b);
}

fn slice_sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn main() {
    println!("{}", add(1, 2));
    println!("{}", slice_sum(&[1, 2, 3]));
    print_modulo(10, 2);
}
