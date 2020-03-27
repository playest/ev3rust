#![feature(prelude_import)]
#![feature(fmt_internals)]
#![feature(box_syntax)]
#[prelude_import]

extern crate ev3dev_lang_rust_derive;
extern crate alloc;

mod ev3;
use ev3::Ev3Result;
use ev3::motors::{DcMotor, Motor, ServoMotor, TachoMotor, MotorPort, LargeMotor};
use ev3::sensors::Sensor;
use ev3::Device;
use ev3::Findable;


fn main() -> Ev3Result<()> {
    let m = LargeMotor::get(MotorPort::OutA)?;
    //m.run_direct(); // this code is valid

    //m.run<ctrl + tab> // completion works
    //LargeMotor::g<ctrl + tab> completion works
    
    Ok(())
}
