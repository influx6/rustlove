pub fn say_hello() {
    println!("hello to the world");
}

pub fn print_numbers() {
    let numbers = [1, 2, 3, 4, 5];
    for n in numbers.iter() {
        println!("{}", n);
    }
}

pub fn output_sequence(target: &[u8]) {
    for n in target {
        println!("{}", n);
    }
}

pub fn generate_sequence(limit: u8) -> Vec<u8> {
    let mut numbers = Vec::new();
    for n in 1..=limit {
        numbers.push(n);
    }
    numbers
}

pub fn generate_sequence2(limit: u8) -> Vec<u8> {
    (1..=limit).collect()
}

#[test]
fn generate_sequence_should_work() {
    let result = generate_sequence(3);
    assert_eq!(result, &[1, 2, 3])
}

#[test]
fn generate_sequence2_should_work() {
    let result = generate_sequence2(3);
    assert_eq!(result, &[1, 2, 3])
}
