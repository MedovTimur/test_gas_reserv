#![no_std]
use gstd::{msg, ReservationId};

const RESERVATION_AMOUNT: u64 = 100_000_000_000;
const RESERVATION_DURATION: u32 = 28_800;
const TIME_FOR_MOVE: u32 = 20;

#[no_mangle]
extern "C" fn handle() {

    let reservation_id = ReservationId::reserve(RESERVATION_AMOUNT, RESERVATION_DURATION)
        .expect("reservation across executions");

    msg::send_delayed_from_reservation(reservation_id, msg::source(), 0, 0, TIME_FOR_MOVE + 1)
        .expect("Error in sending a delayed message");

}
