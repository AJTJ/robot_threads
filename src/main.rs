// use indicatif::ProgressBar;
use rand::prelude::*;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    struct PowerBot {
        power_generated: u64,
        power_increment: u64,
        power_delay: u64,
    }

    struct SupportBot {
        boost_generated: u64,
        boost_increment: u64,
        boost_delay: u64,
    }

    struct PowerState {
        total_power: u64,
        power_bots: u64,
        support_bots: u64,
    }

    let current_state = PowerState {
        total_power: 0,
        power_bots: 3,
        support_bots: 2,
    };

    let shared_state = Arc::new(Mutex::new(current_state));
    // bar.set_position(state.lock().unwrap().total_power);

    let mut all_power_bots = vec![];
    let mut all_support_bots = vec![];

    let msg_senders_arc: Arc<Mutex<Vec<mpsc::Sender<u64>>>> = Arc::new(Mutex::new(vec![]));

    // generate power bots
    let state_arc = Arc::clone(&shared_state);
    for x in 0..state_arc.lock().unwrap().power_bots {
        let (tx, rx) = mpsc::channel();
        let senders_arc = Arc::clone(&msg_senders_arc);
        let mut message_senders = senders_arc.lock().unwrap();
        message_senders.push(tx);
        let state_arc = Arc::clone(&shared_state);
        let bot_handler = thread::spawn(move || {
            let mut new_bot = PowerBot {
                power_generated: 0,
                power_increment: 1,
                power_delay: 100,
            };
            loop {
                let received = rx.try_recv();
                if let Ok(msg) = received {
                    println!("bot {} received {} boost", x, msg);
                    new_bot.power_increment += msg;
                }
                // println!("{}", new_bot.power_increment);
                new_bot.power_generated += new_bot.power_increment;

                thread::sleep(Duration::from_millis(new_bot.power_delay));
                if new_bot.power_generated >= new_bot.power_increment * 10 {
                    let mut state_object = state_arc.lock().unwrap();
                    state_object.total_power += new_bot.power_generated;
                    println!(
                        "{} incremented the total by {}, to: {}",
                        x, new_bot.power_generated, state_object.total_power
                    );
                    new_bot.power_generated = 0;
                }
            }
        });
        all_power_bots.push(bot_handler);
    }

    // generate support bots
    let state_arc = Arc::clone(&shared_state);
    for x in 0..state_arc.lock().unwrap().support_bots {
        let senders_arc = Arc::clone(&msg_senders_arc);
        let bot_handler = thread::spawn(move || {
            let mut new_bot = SupportBot {
                boost_generated: 0,
                boost_increment: 5,
                boost_delay: 500,
            };
            loop {
                if new_bot.boost_generated < 100 {
                    new_bot.boost_generated += new_bot.boost_increment;

                    thread::sleep(Duration::from_millis(new_bot.boost_delay));
                    if new_bot.boost_generated >= new_bot.boost_increment * 3 {
                        let message_senders = senders_arc.lock().unwrap();
                        let mut rng = thread_rng();
                        let senders_index = rng.gen_range(0..message_senders.len());

                        println!(
                            "SUPPORT BOT {}, sent {} boost to {}",
                            x, new_bot.boost_generated, senders_index
                        );
                        match message_senders[senders_index].send(new_bot.boost_generated) {
                            Ok(_) => {}
                            Err(e) => println!("got an err: {}", e),
                        }

                        new_bot.boost_generated = 0;
                    }
                } else {
                    break;
                }
            }
        });
        all_support_bots.push(bot_handler);
    }

    for bot in all_power_bots {
        bot.join().unwrap();
    }
    for bot in all_support_bots {
        bot.join().unwrap();
    }
}
