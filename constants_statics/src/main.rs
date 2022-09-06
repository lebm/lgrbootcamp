#[allow(dead_code)]
// Constant needs an explict type declaration
// Does not use memory. It is inlined when used.
const MAX_PLAYERS: u8 = 10;

#[allow(dead_code)]
// static can be mutable, but it can only be used in unsafe blocks
//static mut CASINO_NAME: &str = "Rusty Casino";
// Does use memory
static CASINO_NAME: &str = "Rusty Casino";

#[allow(unused_variables)]
fn main() {
    let a = MAX_PLAYERS;
    let b = 10;

    let c = CASINO_NAME;
}
