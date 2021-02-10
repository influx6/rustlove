fn main() {
    println!("Hello, world!");
    numbers::say_hello();
    numbers::print_numbers();

    // write a list of u8 array
    // 1 way is:
    let first_style = [1u8, 2, 3, 4, 5];
    println!("{:?}", first_style);

    // 2. another way is declaring the type explicitly
    let second_style: [u8; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", second_style);

    let heap_list = vec![1, 2, 3, 4, 5];
    println!("{:?}", heap_list);

    numbers::output_sequence(&heap_list);
    numbers::output_sequence(&second_style);

    let generated_vec = numbers::generate_sequence(10);
    numbers::output_sequence(&generated_vec);

    let generated_vec2 = numbers::generate_sequence2(10);
    numbers::output_sequence(&generated_vec2);

    println!("done");
}
