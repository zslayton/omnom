#![feature(collections)]
#![feature(core)]
#![feature(std_misc)]

extern crate stomp;

use std::sync::mpsc::{Sender, Receiver, channel, RecvError};
use std::thread::Thread;
use stomp::frame::Frame;
use stomp::subscription::AckOrNack;

const NUMBER_OF_THREADS : u32 = 10;
const REPORT_EVERY : u64 = 1000;
const TOPIC : &'static str = "/queue/QUEUE_NAME";

fn main() {
  let mut receivers : Vec<Receiver<()>> = Vec::with_capacity(NUMBER_OF_THREADS as usize);

  for thread_number in range(0u32, NUMBER_OF_THREADS) {
    let (tx, rx): (Sender<()>, Receiver<()>) = channel();
    receivers.push(rx);
    Thread::spawn(move || {
      let mut message_count : u64 = 0;
      let mut session = match stomp::session("127.0.0.1", 61613).start() {
        Ok(session) => session,
        Err(error) => panic!("Could not connect to the server: {}", error)
      };
      match session.subscription(TOPIC, |frame: &Frame|{
        message_count += 1;
        if message_count % REPORT_EVERY == 0 {
          println!("Thread #{} -> {}", thread_number, message_count);
        }
        AckOrNack::Ack
      }).start() {
        Ok(_) => println!("Thread #{} -> subscribed successfully.", thread_number),
        Err(error) => println!("Thread #{} -> Failed to subscribe: {}", thread_number, error)
      }
      session.listen(); // Loops infinitely, awaiting messages
      tx.send(());
    });
  }
  let results : Vec<Result<(), RecvError>> = receivers.iter().map(|rx| (*rx).recv()).collect();
}
