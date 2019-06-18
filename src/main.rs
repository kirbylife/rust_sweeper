/*
Author: @kirbylife
license: GPLv2
*/


extern crate ncurses;

use ncurses::*;

const ROWS: usize = 15;
const COLUMNS: usize = 10;
const MINES: i32 = ((ROWS * COLUMNS) as i32 * 20 / 100);


fn main() {
    let mut running = true;
    let mut board = [['-'; COLUMNS]; ROWS];
    let mut pivot = [0 as usize; 2];
    initscr();
    raw();
    
    keypad(stdscr(), true);
    noecho();

    while running == true {
        clear();
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
         else if ch == 'q' as i32 || ch == 'Q' as i32 {
            // if you press q, the game ends
            break;
        }
        refresh();
    }
    endwin();
}
