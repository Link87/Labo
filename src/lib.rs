extern crate futures;
extern crate telegram_bot_fork;
extern crate tokio;

use futures::{Stream, future::lazy};
use telegram_bot_fork::*;
use tokio::runtime::current_thread::Runtime;

#[cfg(test)]
mod tests;
mod token;

use token::TELEGRAM_BOT_TOKEN;

#[derive(Debug)]
pub struct Config {
    api_token: String,
    programs: Vec<Program>,
}

impl Config {
    pub fn new() -> Result<Config, ()> {
        Ok(Config {
            //api_token,
            api_token: String::from(TELEGRAM_BOT_TOKEN),
            programs: Vec::new(),
        })
    }
}

struct Washer {
    state: WasherState,
}

enum WasherState {
    Active(Program),
    Finished(Program),
    Idle,
}

#[derive(PartialEq, Debug)]
struct Program {
    name: String,
    temp: u8,
    duration: u16,
}

pub fn run_bot(config: Config) {
    Runtime::new().unwrap().block_on(lazy(|| {
        let api = Api::new(config.api_token).unwrap();

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
                        handle_message(&api, message);
                    }
                }
                Err(err) => eprintln!("{}", err),
            }

            Ok(())
        })
    })).unwrap();
}

pub fn handle_message(api: &Api, message: Message) {
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