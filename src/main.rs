use std::io;


fn main() {
//initializing variables
    let mut board = vec![" ", " ", " ", " ", " ", " ", " ", " ", " "];
    let mut playing = true;
    let mut turn = 1;

    draw_board(&board);

//main game loop
    while playing {

        //check who's turn it is
        if turn % 2 == 0{
            board[(playerO() - 1) as usize] = "O";
        }else{
            board[(playerX() - 1) as usize] = "X";
        }

        //increments the turn and draws the board
        turn += 1;
        draw_board(&board);

        //checks if someone has won
        if check_win(&board) {
            draw_board(&board);
            playing = false;
        }
    }
}

//code for drawing the board
fn draw_board(draw_board: &Vec<&str>) {
    let mut count = 0;

    println!("--------");

    for f in draw_board {
        if count == 3 {
            println!();
            print!("|{}", (f));
            count = 1;
        } else {
            print!("|{}", (f));
            count += 1;
            if count == 3 {
                print!("|");
            }
        }
    }
    println!("\n--------");
}

//code for checking win condition
fn check_win(board:&Vec<&str>) -> bool{
   if board[0] == "X" && board[1] == "X" && board[2] == "X"{
        println!("Congrats Player X Wins!"); return true;
   }else if board[3] == "X" && board[4] == "X" && board[5] == "X"{
       println!("Congrats Player X Wins!"); return true;
   }else if board[6] == "X" && board[7] == "X" && board[8] == "X"{
       println!("Congrats Player X Wins!");return true;
   }else if board[0] == "X" && board[3] == "X" && board[6] == "X"{
       println!("Congrats Player X Wins!");return true;
   }else if board[1] == "X" && board[4] == "X" && board[7] == "X"{
       println!("Congrats Player X Wins!");return true;
   }else if board[2] == "X" && board[5] == "X" && board[8] == "X"{
       println!("Congrats Player X Wins!");return true;
   }else if board[0] == "X" && board[4] == "X" && board[8] == "X"{
       println!("Congrats Player X Wins!");return true;
   }else if board[2] == "X" && board[4] == "X" && board[6] == "X"{
       println!("Congrats Player X Wins!");return true;
   }

   if board[0] == "O" && board[1] == "O" && board[2] == "O"{
        println!("Congrats Player O Wins!");return true;
    }else if board[3] == "O" && board[4] == "O" && board[5] == "O"{
        println!("Congrats Player O Wins!");return true;
    }else if board[6] == "O" && board[7] == "O" && board[8] == "O"{
        println!("Congrats Player O Wins!");return true;
    }else if board[0] == "O" && board[3] == "O" && board[6] == "O"{
        println!("Congrats Player O Wins!");return true;
    }else if board[1] == "O" && board[4] == "O" && board[7] == "O"{
        println!("Congrats Player O Wins!");return true;
    }else if board[2] == "O" && board[5] == "O" && board[8] == "O"{
        println!("Congrats Player O Wins!");return true;
    }else if board[0] == "O" && board[4] == "O" && board[8] == "O"{
        println!("Congrats Player O Wins!");return true;
    }else if board[2] == "O" && board[4] == "O" && board[6] == "O"{
        println!("Congrats Player O Wins!");return true;
    }else if board[0] != " " && board[1] != " " && board[2] != " " && board[3] != " " && board[4] != " " && board[5] != " " && board[6] != " " && board[7] != " " && board[8] != " "{
       println!("Cats no one wins"); return true;
   }else{
       return false;
   }
}

//player X's turn function
fn playerX() -> i32{
    println!("Player X pick a number that represents a square 1 - 9: ");

    let mut selection = String::new();
    io::stdin().read_line(&mut selection).expect("Failed to read line");
    let selection: i32 = selection.trim().parse().expect("Please type a valid number!");

    return selection;
}

//player Y's turn function
fn playerO() -> i32{
    println!("Player O pick a number that represents a square 1 - 9: ");

    let mut selection = String::new();
    io::stdin().read_line(&mut selection).expect("Failed to read line");
    let selection: i32 = selection.trim().parse().expect("Please type a valid number!");

    return selection;
}
