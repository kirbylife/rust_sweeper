/*
Author: @kirbylife
license: GPLv2
*/


extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::Rng;
use std::collections::LinkedList;

const ROWS: usize = 15;
const COLUMNS: usize = 10;
const MINES: i32 = ((ROWS * COLUMNS) as i32 * 20 / 100);

fn get_arround(pivot: [usize; 2]) -> LinkedList<[usize; 2]> {
    // Returns a list with the coordinates around the pivot ignoring the ones that come off the board
    let mut list: LinkedList<[usize; 2]> = LinkedList::new();
    for r in (pivot[0] as i32 - 1)..(pivot[0] as i32 + 2) {
        for c in (pivot[1] as i32- 1)..(pivot[1] as i32 + 2) {
            if r >= 0 && r < ROWS as i32 && c >= 0 && c < COLUMNS as i32 {
                list.push_front([r as usize,c as usize]);
            }
        }
    }
    list
}

fn gen_game(mut counter: i32) -> [[i8; COLUMNS]; ROWS] {
    // returns a matrix with a board already with the mines placed at random
    // -1: box with mine
    // 0-8: box without mine
    let mut rng = rand::thread_rng();
    let mut game = [[0 as i8; COLUMNS]; ROWS];
    while counter != 0 {
        let rnd_row = rng.gen_range(0, ROWS);
        let rnd_column = rng.gen_range(0, COLUMNS);
        if game[rnd_row][rnd_column] == 0 {
            game[rnd_row][rnd_column] = -1;
            counter -= 1;
        }
    }
    for (i, row) in game.clone().iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if item != &-1 {
                let mut counter: i8 = 0;
                for x in get_arround([i, j]).iter() {
                    if game[x[0] as usize][x[1] as usize] == -1 {
                        counter += 1;
                    }
                }
                game[i][j] = counter;
            }
        }
    }
    game
}

fn reveal(pivot: [usize; 2], mut board: [[char; COLUMNS]; ROWS], game: [[i8; COLUMNS]; ROWS]) -> [[char; COLUMNS]; ROWS] {
    // recursively, reveals the zones of the board without mines
    let [row, column] = pivot;
    if game[row][column] != 0 && game[row][column] != -1 {
        board[row][column] = format!("{}", game[row][column]).chars().next().unwrap();
    }
    if game[row][column] == 0 && board[row][column] == '-' {
        board[row][column] = format!("{}", game[row][column]).chars().next().unwrap();
        for tmp_pivot in get_arround(pivot) {
            board = reveal(tmp_pivot, board, game);
        }
    }
    board
}

fn check_revealed(board: [[char; COLUMNS]; ROWS], game: [[i8; COLUMNS]; ROWS]) -> i32 {
    // Return the number of boxes already revealed
    let mut count: i32 = 0;
    for (i, row) in board.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if game[i][j] != -1 && item != &'#' && item != &'-' {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut running = true;
    let mut board = [['-'; COLUMNS]; ROWS];
    let game = gen_game(MINES);
    let mut pivot = [0 as usize; 2];
    initscr();
    raw();
    
    keypad(stdscr(), true);
    noecho();

    while running == true {
        clear();
        let revealed = check_revealed(board, game);
        if revealed == ((COLUMNS * ROWS) as i32 - MINES)  {
            print!("You win");
            break;
        }
        for (i, row) in board.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if pivot[0] == i && pivot[1] == j {
                    addstr(format!("<{}>", item).as_ref());
                }
                else {
                    addstr(format!(" {} ", item).as_ref());
                }
            }
            match i { 
                0 => addstr("\t To move use the arrow keys"),
                1 => addstr("\t To flag a box, press 'f'"),
                2 => addstr("\t To reveal a box, press 'Space'"),
                3 => addstr("\t To exit, press 'q'"),
                _ => addstr(""),
            };
            addstr("\n");
        }
        addstr(format!("{} - {}\n", pivot[0], pivot[1]).as_ref());
        let ch = getch();
        if ch == KEY_UP {
            // Go up
            if pivot[0] > 0 {
                pivot[0] -= 1;
            }
        }
        else if ch == KEY_DOWN {
            // Go down
            if pivot[0] < ROWS-1 {
                pivot[0] += 1;
            }
        }
        else if ch == KEY_LEFT {
            // Go left
            if pivot[1] > 0 {
                pivot[1] -= 1;
            }
        }
        else if ch == KEY_RIGHT {
            // Go Right
            if pivot[1] < COLUMNS-1 {
                pivot[1] += 1;
            }
        }
        else if ch == 'f' as i32 || ch == 'F' as i32 {
            // Flag a box with possibly a mine
            if board[pivot[0]][pivot[1]] == '#' {
                board[pivot[0]][pivot[1]] = '-';
            }
            else if board[pivot[0]][pivot[1]] == '-' {
                board[pivot[0]][pivot[1]] = '#';
            }
        }
        else if ch == ' ' as i32 {
            // Reveal a box
            if board[pivot[0]][pivot[1]] == '-' && game[pivot[0]][pivot[1]] == 0 {
                // If a box with a 0 is revealed, the entire area around it is revealed
                board = reveal(pivot, board, game);
            }
            else if board[pivot[0]][pivot[1]] == '-' && game[pivot[0]][pivot[1]] != -1 {
                // If a box with a value other than 0 and -1 is revealed, only that box is revealed
                board[pivot[0]][pivot[1]] = format!("{}", game[pivot[0]][pivot[1]]).chars().next().unwrap();
            }
            else if board[pivot[0]][pivot[1]] != '#' && game[pivot[0]][pivot[1]] != 0 && game[pivot[0]][pivot[1]] != -1 {
                // If space is pressed in a box already revealed, its neighboring cells are revealed
                let mut count = 0;
                for [r, c] in get_arround(pivot) {
                    if board[r][c] == '#' {
                        count += 1;
                    }
                }
                if count == game[pivot[0]][pivot[1]] {
                    // It's only relieved if the number of boxes around it has already been flagged
                    for [r, c] in get_arround(pivot) {
                        if  board[r][c] != '#' && game[r][c] == -1 {
                            // If a flag was in an incorrect box, the game is over
                            print!("Game Over");
                            running = false;
                            break;
                        }
                        else if board[r][c] != '#' && game[r][c] != 0 {
                            board[r][c] = format!("{}", game[r][c]).chars().next().unwrap();
                        }
                        else if game[r][c] == 0 {
                            // If a box around it had a 0, that whole area is revealed
                            board = reveal([r, c], board, game);
                        }
                    }
                } 
            }
            else if board[pivot[0]][pivot[1]] == '-' && game[pivot[0]][pivot[1]] == -1 {
                // If a box with a mine is revealed, the game is over
                print!("Game over");
                break;
            }
        }

        else if ch == 'q' as i32 || ch == 'Q' as i32 {
            // if you press q, the game ends
            break;
        }
        refresh();
    }
    endwin();
}
