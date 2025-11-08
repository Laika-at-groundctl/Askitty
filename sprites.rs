// where we organise sprites and animations

use std::{thread, time};

pub fn normal() {
    let x = true;
    let frameOne = r" ^___^       _
( -.- )_____/ )
 \          )/
 (          )
  \/------\/";

    let frameTwo = r"
 ^___^       _
( -.- )_____/ )
 \          )/
  \/-------\/";

    let animtime = time::Duration::from_millis(1000);
    while x == true {
        // animation loop
        println!("{}", frameOne);
        thread::sleep(animtime);
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", frameTwo);
        thread::sleep(animtime);
        print!("\x1b[2J\x1B[1;1H");

    }
}
