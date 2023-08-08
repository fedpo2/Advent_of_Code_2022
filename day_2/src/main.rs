use std::fs::read_to_string;


fn main() {
    let resultado_parte1 = procesar_turnos(vectorizar_txt("input.txt"));
    println!("Resultado Parte 1: {}", resultado_parte1);


}


fn vectorizar_txt(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn procesar_turnos(turns: Vec<String>) -> u32 {
    let mut mis_puntos: u32 = 0;

    for turn in turns {
        mis_puntos += resultado_turno(turn);
    }

    mis_puntos
}

fn resultado_turno(turn: String) -> u32 {
    let ret: u32;
    match turn.chars().next().unwrap() {
        'A' => match turn.chars().last().unwrap() {
            'X' => ret = 4,
            'Y' => ret = 8,
            'Z' => ret = 3,
            _ => ret = 0,
        }

        'B' => match turn.chars().last().unwrap() {
            'X' => ret = 1,
            'Y' => ret = 5,
            'Z' => ret = 9,
            _ => ret = 0,
        }

        'C' => match turn.chars().last().unwrap() {
            'X' => ret = 7,
            'Y' => ret = 2,
            'Z' => ret = 6,
            _ => ret = 0,
        }
        _ => ret = 0,
    }
    ret
}
