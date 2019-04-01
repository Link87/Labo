use futures::{future, Stream};
use telegram_bot_fork::*;
use tokio::runtime::current_thread::Runtime;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

mod client;
mod timer;
mod washer;

use client::Client;
use washer::*;

pub fn run_bot() {
    let token = fs::read_to_string("token.key").expect("Couln't load bot api token from file");
    let washer = Rc::new(RefCell::new(Washer::new()));
    let mut clients = HashMap::new();

    Runtime::new()
        .unwrap()
        .block_on(future::lazy(|| {
            let api = Api::new(token).unwrap();

            // Convert stream to the stream with errors in result
            let stream = api.stream().then(|mb_update| {
                let res: Result<Result<Update, Error>, ()> = Ok(mb_update);
                res
            });

            // Print update or error for each update.
            stream.for_each({
                move |update| {
                    match update {
                        Ok(update) => {
                            // If the received update contains a new message...
                            if let UpdateKind::Message(message) = update.kind {
                                let id = message.from.id;
                                let client = clients.entry(id).or_insert_with(|| {
                                    Client::new(message.from.clone(), Rc::clone(&washer))
                                });
                                client.handle_message(api.clone(), message);
                            }
                        }
                        Err(e) => eprintln!("{:?}", e),
                    }
                    Ok(())
                }
            })
        }))
        .unwrap();
}
