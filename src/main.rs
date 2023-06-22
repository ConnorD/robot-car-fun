#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
// use arduino_hal::simple_pwm::Timer0Pwm;
use panic_halt as _;


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    let mut speed_pin_right = pins.d9.into_output().into_pwm(&timer1);
    let mut speed_pin_left = pins.d6.into_output().into_pwm(&timer0);

    let mut right_motor_dir1 = pins.d12.into_output();
    let mut right_motor_dir2 = pins.d11.into_output();

    let mut left_motor_dir1 = pins.d7.into_output();
    let mut left_motor_dir2 = pins.d8.into_output();

    speed_pin_right.enable();
    speed_pin_right.set_duty(200);

    speed_pin_left.enable();
    speed_pin_left.set_duty(200);

    // let mut led = pins.d13.into_output();

    loop {
        right_motor_dir1.toggle();
        left_motor_dir1.toggle();
        arduino_hal::delay_ms(1000);
        right_motor_dir2.toggle();
        left_motor_dir2.toggle();
    }
}
