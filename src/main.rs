extern crate futures;
extern crate telegram_bot_fork;
extern crate tokio;

use futures::{Stream, future::lazy};
use telegram_bot_fork::*;
use tokio::runtime::current_thread::Runtime;
use std::env;

use labo;

fn main() {
    Runtime::new().unwrap().block_on(lazy(|| {
        let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
        let api = Api::new(token).unwrap();

        // Convert stream to the stream with errors in result
        let stream = api.stream().then(|mb_update| {
            let res: Result<Result<Update, Error>, ()> = Ok(mb_update);
            res
        });

        // Print update or error for each update.
        stream.for_each(move |update| {
            match update {
                Ok(update) => {
                    // If the received update contains a new message...
                    if let UpdateKind::Message(message) = update.kind {
                        if let MessageKind::Text { ref data, .. } = message.kind {
                            // Print received text message to stdout.
                            println!("<{}>: {}", &message.from.first_name, data);

                            // Answer message with "Hi".
                            api.spawn(message.text_reply(format!(
                                "Hi, {}! You just wrote '{}'",
                                &message.from.first_name, data
                            )));
                        }
                    }
                }
                Err(_) => {}
            }

            Ok(())
        })
    })).unwrap();
}
