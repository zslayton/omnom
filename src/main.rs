extern crate stomp;
use stomp::frame::Frame;
use stomp::subscription::{Ack, AckOrNack, Auto};

const NUMBER_OF_THREADS : uint = 10;
const TOPIC : &'static str = "/queue/QUEUE_NAME";

fn main() {
  
  fn on_message(frame: &Frame) -> AckOrNack {
    Ack
  }

  let mut receivers : Vec<Receiver<()>> = Vec::with_capacity(NUMBER_OF_THREADS);

  for thread_number in range(0, NUMBER_OF_THREADS) {
    let (tx, rx): (Sender<()>, Receiver<()>) = channel();
    receivers.push(rx);
    spawn(proc() {
      let mut session = match stomp::connect("127.0.0.1", 61613) {
        Ok(session)  => session,
        Err(error) => panic!("Could not connect to the server: {}", error)
      };
      session.subscribe(TOPIC, Auto, on_message);
      println!("Session #{} created. Subscribed to {}", thread_number, TOPIC);
      session.listen(); // Loops infinitely, awaiting messages
      tx.send(());
    });
  }

  let _ = receivers.iter().map(|rx| (*rx).recv());
}
