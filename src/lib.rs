#![no_std]

use gstd::{msg, prelude::*};

mod io;

static mut VALUE: bool = false;

#[no_mangle]
unsafe extern "C" fn handle() {
    let input: String = msg::load().expect("Failed to load input");

    if input != String::from("Ping") {
        let reply: Result<String, u16> = Err(1);
        msg::reply(reply, 0).expect("Failed to reply");
    } else {
        VALUE = !VALUE;

        if VALUE {
            let reply: Result<String, u16> = Ok(String::from("Pong"));
            msg::reply(reply, 0).expect("Failed to reply");
        } else {
            let reply: Result<String, u16> = Err(1);
            msg::reply(reply, 0).expect("Failed to reply");
        }
    }
}
