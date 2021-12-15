#![no_std]
#![no_main]
#![macro_use]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

use drogue_device::{
    actors::button::{Button, ButtonPressed},
    actors::led::matrix::{AnimationEffect, MatrixCommand},
    bsp::boards::nrf52::microbit::*,
    ActorContext, Board,
};

use embassy::time::Duration;
use embassy_nrf::Peripherals;

#[embassy::main]
async fn main(spawner: embassy::executor::Spawner, p: Peripherals) {
    // Using a board support package to simplify setup
    let board = Microbit::new(p);

    // Led Matrix actor that will handle the display refresh loop and state of LED matrix
    static LED_MATRIX: ActorContext<LedMatrixActor> = ActorContext::new();

    // Mounting will start the display loop
    let matrix = LED_MATRIX.mount(spawner, LedMatrixActor::new(board.led_matrix, None));

    // An actor for the game logic
    let text = "{{display_text}}";

    // Actor for button 'A' that will scroll text whenever pushed.
    static BUTTON_A: ActorContext<Button<ButtonA, ButtonPressed<LedMatrixActor>>> =
        ActorContext::new();
    BUTTON_A.mount(
        spawner,
        Button::new(
            board.button_a,
            ButtonPressed(
                matrix,
                MatrixCommand::ApplyText(
                    text,
                    AnimationEffect::Slide,
                    Duration::from_secs((text.len() / 2) as u64),
                ),
            ),
        ),
    );
}
