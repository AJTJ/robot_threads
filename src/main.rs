use indicatif::ProgressBar;
use rand::prelude::*;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    enum BotKind {
        Power,
        Support,
    }

    struct PowerBot {
        kind: BotKind,
        power_generated: u64,
        power_increment: u64,
        power_delay: u64,
    }

    struct SupportBot {
        kind: BotKind,
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

    let (tx, rx) = mpsc::channel();

    // generate support bots
    let state_arc = Arc::clone(&shared_state);
    for x in 0..state_arc.lock().unwrap().support_bots {
        let state_arc = Arc::clone(&shared_state);
        let sender = tx.clone();
        let bot_handler = thread::spawn(move || {
            let mut new_bot = SupportBot {
                kind: BotKind::Support,
                boost_generated: 0,
                boost_increment: 5,
                boost_delay: 500,
            };
            loop {
                if new_bot.boost_generated < 100 {
                    new_bot.boost_generated += new_bot.boost_increment;

                    thread::sleep(Duration::from_millis(new_bot.boost_delay));
                    if new_bot.boost_generated >= new_bot.boost_increment * 3 {
                        let mut state_object = state_arc.lock().unwrap();
                        let mut rng = thread_rng();
                        let val = rng.gen_range(1..100);
                        sender.send(val).unwrap();

                        new_bot.boost_generated = 0;
                    }
                } else {
                    break;
                }
            }
        });
        all_support_bots.push(bot_handler);
    }

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // generate power bots
    let state_arc = Arc::clone(&shared_state);
    for x in 0..state_arc.lock().unwrap().power_bots {
        let state_arc = Arc::clone(&shared_state);
        let bot_handler = thread::spawn(move || {
            let mut new_bot = PowerBot {
                kind: BotKind::Power,
                power_generated: 0,
                power_increment: 1,
                power_delay: 100,
            };
            loop {
                if new_bot.power_generated < 100 {
                    new_bot.power_generated += new_bot.power_increment;

                    thread::sleep(Duration::from_millis(new_bot.power_delay));
                    if new_bot.power_generated >= new_bot.power_increment * 10 {
                        let mut state_object = state_arc.lock().unwrap();
                        state_object.total_power += new_bot.power_generated;
                        println!(
                            "{} incremented the total by {}, to: {}",
                            x, new_bot.power_increment, state_object.total_power
                        );
                        new_bot.power_generated = 0;
                    }
                } else {
                    break;
                }
            }
        });
        all_power_bots.push(bot_handler);
    }

    for bot in all_power_bots {
        bot.join().unwrap();
    }
    for bot in all_support_bots {
        bot.join().unwrap();
    }
}

// let target_power = 1_000_000_000;
// let bar = ProgressBar::new(target_power);
// // for _ in 0..1000 {
// //     bar.inc(1);
// //     thread::sleep(Duration::from_millis(1));
// //     // ...
// // }
// loop {
//     let state = Arc::clone(&shared_state);
//     let mut state = state.lock().unwrap();
//     bar.set_position(state.total_power);
// }
