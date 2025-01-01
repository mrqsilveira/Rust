#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
use TrafficLight::{Red, Yellow};
fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    match red  {
        Red =>  println!("colour is {:?}.", red),
        _ => (),
    }
   
}

