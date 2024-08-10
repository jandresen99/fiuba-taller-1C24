#[derive(Debug)]
pub enum ChessError {
    TypeDoesntExists,
}

/// Estructura utilizada para guardar la posición de la pieza en un tablero de 8x8
pub struct Position {
    x: u8,
    y: u8,
}

/// Estructura principal que guarda distintas propiedades de la pieza, como la posición, el tipo de pieza y el color
pub struct Piece {
    position: Position,
    piece_type: PieceType,
    color: PieceColor,
}

/// Variaciones de los tipos de pieza
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

/// Variaciones de los colores de la pieza
pub enum PieceColor {
    Black,
    White,
}

// GET_PIECE: Genera una nueva pieza basandose en los datos recibidos
pub fn get_piece(letter: char, x: u8, y: u8) -> Result<Piece, ChessError> {
    let piece_type;
    let piece_position = Position { x: x + 1, y: 8 - y };
    let mut piece_color = PieceColor::Black;
    match letter {
        'r' => piece_type = PieceType::King,
        'd' => piece_type = PieceType::Queen,
        'a' => piece_type = PieceType::Bishop,
        'c' => piece_type = PieceType::Knight,
        't' => piece_type = PieceType::Rook,
        'p' => piece_type = PieceType::Pawn,
        'R' => piece_type = PieceType::King,
        'D' => piece_type = PieceType::Queen,
        'A' => piece_type = PieceType::Bishop,
        'C' => piece_type = PieceType::Knight,
        'T' => piece_type = PieceType::Rook,
        'P' => piece_type = PieceType::Pawn,
        _ => {
            println!("ERROR: El tipo de pieza ingrasado no existe");
            return Err(ChessError::TypeDoesntExists);
        }
    }

    if letter.is_lowercase() {
        piece_color = PieceColor::White;
    }

    let new_piece = Piece {
        position: piece_position,
        piece_type: piece_type,
        color: piece_color,
    };

    return Ok(new_piece);
}

/// CAN_CAPTURE: Funcion que determina si una pieza puede capturar a otra basandose en su posicion y el tipo de pieza
/// Se calcula la diferencia entre las posiciones para x e y
/// Segun el tipo de pieza vamos a realizar distintas verificaciones para determinar si una pieza pueda capturar a otra
/// - REY: si la diferencia entre las posiciones x e y son menores o iguales a uno, puede capturar
/// - DAMA: si se encuentran en la misma columna, o fila o diagonal, puede capturar
/// - ALFIL: si se encuentan en la misma diagonal, puede capturar
/// - CABALLO: si las diferencias forman una L, puede capturar
/// - TORRE: si se encuentran en la misma fila o columna, puede capturar
/// - PEON: si estan en la misma columna y a uno o dos (si esta en la posicion inicial) movimientos, puede capturar
pub fn can_capture(first_piece: &Piece, second_piece: &Piece) -> bool {
    let fx = first_piece.position.x;
    let fy = first_piece.position.y;
    let sx = second_piece.position.x;
    let sy = second_piece.position.y;

    let x_diff = (fx as i8 - sx as i8).abs() as u8;
    let y_diff = (fy as i8 - sy as i8).abs() as u8;

    match first_piece.piece_type {
        PieceType::King => x_diff <= 1 && y_diff <= 1,
        PieceType::Queen => x_diff == 0 || y_diff == 0 || x_diff == y_diff,
        PieceType::Bishop => x_diff == y_diff,
        PieceType::Knight => (x_diff == 2 && y_diff == 1) || (x_diff == 1 && y_diff == 2),
        PieceType::Rook => x_diff == 0 || y_diff == 0,
        PieceType::Pawn => {
            let y_diff_aux = (fy as i8 - sy as i8) as i8;

            match first_piece.color {
                PieceColor::Black => x_diff == 1 && y_diff_aux == 1,
                PieceColor::White => x_diff == 1 && y_diff_aux == -1,
            }
        }
    }
}

// GET_RESULT: imprime el resultado basandose en los resultados obtenidos luego de realizar un can_capture
pub fn get_result(
    can_capture1: bool,
    can_capture2: bool,
    first_piece: &Piece,
    second_piece: &Piece,
) -> String {
    if can_capture1 && can_capture2 {
        return "E".to_string();
    } else if can_capture1 && !can_capture2 {
        match first_piece.color {
            PieceColor::White => {
                return "B".to_string();
            }
            PieceColor::Black => {
                return "N".to_string();
            }
        }
    } else if !can_capture1 && can_capture2 {
        match second_piece.color {
            PieceColor::White => {
                return "B".to_string();
            }
            PieceColor::Black => {
                return "N".to_string();
            }
        }
    } else {
        return "P".to_string();
    }
}

/// PROCESS_BOARD: recibe las filas del archivo leido y devueve un vector con las piezas generadas
pub fn process_board(rows: Vec<String>) -> Result<Vec<Piece>, ChessError> {
    let mut pieces: Vec<Piece> = Vec::new();

    for (y, row) in rows.iter().enumerate() {
        if let Ok(y) = y.try_into() {
            let trimmed_row = row.replace(" ", "");
            for (x, c) in trimmed_row.chars().enumerate() {
                if let Ok(x) = x.try_into() {
                    if c != '_' {
                        if let Ok(piece) = get_piece(c, x, y) {
                            pieces.push(piece);
                        } else {
                            println!("Error: fallo en get_piece");
                        }
                    }
                } else {
                    println!("Error: fallo el enumerate sobre la variable trimmed_row");
                }
            }
        } else {
            println!("Error: fallo el enumerate sobre la variable rows");
        }
    }
    return Ok(pieces);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;

    #[test]
    fn position_test() {
        let position = Position { x: 1, y: 8 };

        assert_eq!(position.x, 1);
        assert_eq!(position.y, 8);
    }

    #[test]
    fn get_piece_test() {
        let letter = 'a';
        let x = 1;
        let y = 0;

        let new_piece = get_piece(letter, x, y).unwrap();

        assert_eq!(new_piece.position.x, 2);
        assert_eq!(new_piece.position.y, 8);
        assert!(matches!(new_piece.piece_type, PieceType::Bishop));
        assert!(matches!(new_piece.color, PieceColor::White));
    }

    #[test]
    fn get_piece_type_error_test() {
        let letter = 'x';
        let x = 1;
        let y = 0;

        let new_piece = get_piece(letter, x, y);

        assert!(new_piece.is_err());
    }

    #[test]
    fn can_capture_true_test() {
        let first_piece = get_piece('r', 0, 0).unwrap();
        let second_piece = get_piece('r', 0, 1).unwrap();

        assert!(can_capture(&first_piece, &second_piece));
    }

    #[test]
    fn can_capture_false_test() {
        let first_piece = get_piece('r', 0, 0).unwrap();
        let second_piece = get_piece('r', 3, 4).unwrap();

        assert_eq!(can_capture(&first_piece, &second_piece), false);
    }

    #[test]
    fn process_board_test() {
        let mut rows = Vec::new();

        rows.push("_ _ _ _ _ _ _ _".to_string());
        rows.push("_ _ _ _ _ _ _ _".to_string());
        rows.push("_ _ _ D _ _ _ _".to_string());
        rows.push("_ _ _ _ _ _ _ _".to_string());
        rows.push("_ _ _ _ _ _ _ _".to_string());
        rows.push("_ _ _ _ _ _ t _".to_string());
        rows.push("_ _ _ _ _ _ _ _".to_string());
        rows.push("_ _ _ _ _ _ _ _".to_string());

        let pieces = process_board(rows);

        assert_eq!(pieces.unwrap().len(), 2);
    }

    #[test]
    fn get_result_test() {
        let first_piece = get_piece('r', 0, 0).unwrap();
        let second_piece = get_piece('R', 0, 1).unwrap();

        assert_eq!(
            get_result(true, true, &first_piece, &second_piece),
            "E".to_string()
        );
    }
}
