use ajedrez::chess;

#[test]
fn board1_test() {
    let mut rows = Vec::new();

    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ D _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ t _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());

    let pieces = chess::process_board(rows).unwrap();

    let first_piece = &pieces[0];
    let second_piece = &pieces[1];

    let can_capture1 = chess::can_capture(first_piece, second_piece);
    let can_capture2 = chess::can_capture(second_piece, first_piece);

    assert_eq!(
        chess::get_result(can_capture1, can_capture2, &first_piece, &second_piece),
        "N".to_string()
    );
}

#[test]
fn board2_test() {
    let mut rows = Vec::new();

    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ P _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ a _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());

    let pieces = chess::process_board(rows).unwrap();

    let first_piece = &pieces[0];
    let second_piece = &pieces[1];

    let can_capture1 = chess::can_capture(first_piece, second_piece);
    let can_capture2 = chess::can_capture(second_piece, first_piece);

    assert_eq!(
        chess::get_result(can_capture1, can_capture2, &first_piece, &second_piece),
        "B".to_string()
    );
}

#[test]
fn board3_test() {
    let mut rows = Vec::new();

    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ R _ _ _ _ _".to_string());
    rows.push("_ _ t _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());

    let pieces = chess::process_board(rows).unwrap();

    let first_piece = &pieces[0];
    let second_piece = &pieces[1];

    let can_capture1 = chess::can_capture(first_piece, second_piece);
    let can_capture2 = chess::can_capture(second_piece, first_piece);

    assert_eq!(
        chess::get_result(can_capture1, can_capture2, &first_piece, &second_piece),
        "E".to_string()
    );
}

#[test]
fn board4_test() {
    let mut rows = Vec::new();

    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ P _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ d _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());

    let pieces = chess::process_board(rows).unwrap();

    let first_piece = &pieces[0];
    let second_piece = &pieces[1];

    let can_capture1 = chess::can_capture(first_piece, second_piece);
    let can_capture2 = chess::can_capture(second_piece, first_piece);

    assert_eq!(
        chess::get_result(can_capture1, can_capture2, &first_piece, &second_piece),
        "P".to_string()
    );
}

#[test]
fn board5_test() {
    let mut rows = Vec::new();

    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ p _ _ _ _ _".to_string());
    rows.push("_ _ _ P _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());

    let pieces = chess::process_board(rows).unwrap();

    let first_piece = &pieces[0];
    let second_piece = &pieces[1];

    let can_capture1 = chess::can_capture(first_piece, second_piece);
    let can_capture2 = chess::can_capture(second_piece, first_piece);

    assert_eq!(
        chess::get_result(can_capture1, can_capture2, &first_piece, &second_piece),
        "P".to_string()
    );
}

#[test]
fn board6_test() {
    let mut rows = Vec::new();

    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ P _ _ _ _ _".to_string());
    rows.push("_ _ _ p _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());
    rows.push("_ _ _ _ _ _ _ _".to_string());

    let pieces = chess::process_board(rows).unwrap();

    let first_piece = &pieces[0];
    let second_piece = &pieces[1];

    let can_capture1 = chess::can_capture(first_piece, second_piece);
    let can_capture2 = chess::can_capture(second_piece, first_piece);

    assert_eq!(
        chess::get_result(can_capture1, can_capture2, &first_piece, &second_piece),
        "E".to_string()
    );
}
