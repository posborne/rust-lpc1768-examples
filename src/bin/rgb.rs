// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.
#![feature(plugin, no_std, core, start)]
#![no_std]
#![plugin(macro_platformtree)]

extern crate core;
extern crate zinc;

use zinc::hal::timer::Timer;
use zinc::hal::pin::Gpio;

// This example shows use of the RGB LED that is availble on the MBED
// Application Board.  The LED is connected to 3 pins coming
// from the MBED LPC1768.  Here's the mapping:
//
// - RGB_RED   => p23 => p2.3 (PWM1.4)
// - RGB_GREEN => p24 => p2.2 (PWM1.3)
// - RGB_BLUE  => p25 => p2.1 (PWM1.2)

platformtree!(
    lpc17xx@mcu {
        clock {
            source = "main-oscillator";
            source_frequency = 12_000_000;
            pll {
                m = 50;
                n = 3;
                divisor = 4;
            }
        }

        timer {
            timer@1 {
                counter = 25;
                divisor = 4;
            }
        }

        gpio {
            2 {
                rgb_red@3 { direction = "out"; }
                rgb_green@2 { direction = "out"; }
                rgb_blue@1 { direction = "out"; }
            }
        }
    }

    os {
        single_task {
            loop = "run";
            args {
                timer = &timer;
                rgb_red = &rgb_red;
                rgb_green = &rgb_green;
                rgb_blue = &rgb_blue;
            }
        }
    }
);


fn drive_pwm(timer: &Timer, gpio: &Gpio, period_us: u32, pulsewidth_us: u32, duration_us: u32) {
    // drive pwm for at least the specified duration_us
    let num_cycles: u32 = (duration_us + (period_us - 1)) / period_us;
    for _ in 0..num_cycles {
        gpio.set_low();
        timer.wait_us(pulsewidth_us);
        gpio.set_high();
        timer.wait_us(period_us - pulsewidth_us);
    }
} 


fn do_color(timer: &Timer, gpio: &Gpio) {
    for i in 0..100 {
        drive_pwm(timer, gpio, 100, i, 10_000); // 10ms
    }
}

fn run(args: &pt::run_args) {
    for &gpio in &[args.rgb_red, args.rgb_green, args.rgb_blue] {
        args.rgb_red.set_high();
        args.rgb_green.set_high();
        args.rgb_blue.set_high();

        do_color(args.timer, gpio);
    }
}
