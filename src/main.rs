use std::{io::{stdin, stdout, Write}, process::exit, str::FromStr};
use chess::{Board, ChessMove, MoveGen};
use vampirc_uci::{parse_one, Serializable, UciMessage, UciTimeControl};

fn main() {
    println!("hehez");
    let mut board: Board = Board::default();
    loop {
        let mut input = String::new();
        let mut out: UciMessage;
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        let mesg: UciMessage = parse_one(&input);
        match mesg {
            UciMessage::Uci => {
                // id
                out = UciMessage::Id { name: Some("sortfish".to_string()), author: Some("josh tenorio".to_string()) };
                println!("{}", out);
                // TODO option

                // uciok
                out = UciMessage::UciOk;
                println!("{}", out);
            }
            UciMessage::IsReady => {
                out = UciMessage::ReadyOk;
                println!("{}", out);
            }
            UciMessage::Debug(_debug) => {
                // ???
            }
            UciMessage::Quit => {
                exit(0);
            }
            UciMessage::Position { startpos, fen, moves } => {
                if !startpos {
                    // parse fen
                    match fen {
                        Some(uci_fen) => {
                            let fen_str = uci_fen.0;
                            board = Board::from_str(fen_str.as_str()).unwrap(); // TODO don't unwrap
                        },
                        None => {
                            // ???
                        }
                    }
                }
                else {
                    board = Board::default();
                }
                println!("moves len: {}", moves.len());
                for chess_move in moves {
                    board = board.make_move_new(chess_move);
                    println!("side to move: {:?}", board.side_to_move())
                }
            }
            UciMessage::Go { time_control, search_control } => {

                // TODO already sorted alphabetically?
                let iterable = MoveGen::new_legal(&board);
                let mut chosen_move: ChessMove = ChessMove::default();
                let num_moves = iterable.len();
                println!("legal moves: {}", num_moves);
                let mut cntr = 0;
                let target = num_moves / 2;
                for mv in iterable {
                    if cntr == target {
                        chosen_move = mv.clone();
                        break
                    }
                    cntr += 1;
                }
                if num_moves == 0 {
                    // TODO retire
                }

                out = UciMessage::best_move(chosen_move);
                println!("{}", out);
            }
            _ => {
                todo!("arm not implemented")
            }
        }
    }
}
