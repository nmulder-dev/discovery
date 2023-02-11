#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut grid = [
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    loop {
        // Show light_it_all for 1000ms
        display.show(&mut timer, grid, 1000);
        let mut pos: (u8, u8) = (0,0);

        // Get lit node
        for (i, row) in grid {
            for (j, col) in row {
                if col == 1 {
                    pos = (i, j);
                }
            }
        }

        timer.delay_ms(1000_u32);
    }
}