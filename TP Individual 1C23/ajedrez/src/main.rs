use ajedrez::chess;
use ajedrez::utils::read_lines;
use std::env;

// MAIN: funcion principal
fn main() {
    let mut rows = Vec::new();

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("ERROR: Faltan argumentos");
    }

    let first_arg = &args[1];

    match read_lines(first_arg.to_string()) {
        Ok(lines) => {
            for line in lines {
                if let Ok(line) = line {
                    rows.push(line);
                }
            }
        }
        Err(error) => println!("ERROR: {}", error),
    }

    let pieces = chess::process_board(rows);

    if let Ok(pieces) = pieces {
        let can_capture1 = chess::can_capture(&pieces[0], &pieces[1]);
        let can_capture2 = chess::can_capture(&pieces[1], &pieces[0]);

        println!(
            "{}",
            chess::get_result(can_capture1, can_capture2, &pieces[0], &pieces[1])
        );
    }
}
