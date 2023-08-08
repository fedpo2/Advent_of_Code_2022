use std::fs::read_to_string;

fn main() {
    let mut elf_values = decode_elf_calories(vectorizar_txt("input.txt"));
    elf_values.sort();
    println!("{}", elf_values[elf_values.len()-1]);
}

fn vectorizar_txt(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn decode_elf_calories(lines: Vec<String>) -> Vec<u32> {
    let mut valores: Vec<u32> = Vec::new();
    let mut accumulator: u32 = 0;

    for line in lines {
        if line.is_empty() {
            valores.push(accumulator);
            accumulator = 0;
            continue;
        }
        accumulator += line.parse::<u32>().unwrap();
    }

    valores
}
