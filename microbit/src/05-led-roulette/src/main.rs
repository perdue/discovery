#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut led = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let N = 5;
    loop {
        let mut row;
        let mut col;
        for i in 0..16 {
            if i < N {
                row = 0;
                col = i;
                if col > 0 {
                    led[row][col-1] = 0;
                }
                led[row][col] = 1;
            } else if N < i && i < 10 {
                row = (i-N)%N;
                col = N - 1;
                if row > 0 {
                    led[row-1][col] = 0;
                }
                led[row][col] = 1;
            }

            display.show(&mut timer, led, 100);

            display.clear();

            timer.delay_ms(100_u32);
        }
    }
}
