// where we organise sprites and animations

use std::process::Command;

pub fn normal() {
    let mut x = true;
    let frameOne = r" ^___^       _
( -.- )_____/ )
 \          )/
 (          )
  \/------\/";
    while x == true {
        // animation loop
        println!("{}", frameOne);
        Command::new("clear");
        .spawn();
        .except("failed to clear line");

    }
}
