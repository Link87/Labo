use futures::{future, Stream};
use telegram_bot_fork::*;
use tokio::prelude::*;
use tokio::runtime::current_thread::{self, Runtime};

use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::time::Duration;

mod washer;
mod timer;
mod user;

use washer::*;
use timer::Timer;

pub fn run_bot() {
    let token = fs::read_to_string("token.key")
        .expect("Couln't load bot api token from file");
    let washer = Rc::new(RefCell::new(Washer::new()));

    Runtime::new().unwrap().block_on(future::lazy(|| {
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
                            handle_message(api.clone(), message, Rc::clone(&washer));
                        }
                    },
                    Err(e) => eprintln!("{:?}", e),
                }

                Ok(())
            }
        })
    })).unwrap();
}

/// Analyzes the given message and calls code according to its content.
/// Messages not being text messages are ignored.
fn handle_message(api: Api, message: Message, washer: Rc<RefCell<Washer>>) {
    if let MessageKind::Text { ref data, ref entities } = message.kind {
        // Print received text message to stdout.
        println!("<{}>: {}", &message.from.first_name, data);

        // test if message contains command
        if let Some(cmd_entity) = entities.iter()
            .find(|entity| entity.kind == MessageEntityKind::BotCommand)
        {
            let cmd: String = data.chars().skip(cmd_entity.offset as usize)
                    .take(cmd_entity.length as usize).collect();
            println!("Received command: {}", cmd);
            {
                println!("Washer {:?}", washer.borrow());
            }
            match cmd.as_ref() {
                "/start" => start_laundry(api, message.chat, washer,
                        data.chars()
                        .skip((cmd_entity.offset + cmd_entity.length) as usize)
                        .collect()),
                "/stop" => stop_laundry(washer),
                "/status" => laundry_status(),
                _ => {},
            }
        }

        // Answer message with "Hi".
        /*api.spawn(message.text_reply(format!(
            "Hi, {}! You just wrote '{}'",
            &message.from.first_name, data
        )));*/
    }
}

fn start_laundry(api: Api, chat: MessageChat, washer: Rc<RefCell<Washer>>, program: String) {
    let timer: Timer;
    {
        let mut washer = washer.borrow_mut();
        match washer.state() {
            WasherState::Idle => {},
            _ => return,
        }

        match program.trim() {
            "Baumwolle 30" => {
                timer = washer.start(&Program::new(String::from("Baumwolle 30°C"), Duration::from_secs(5)));
            }
            _ => return,
        }
    }

    let timer = timer.and_then(move |succeeded| {
            if succeeded {
                api.spawn(chat.text("Your laundry is done"));
                washer.borrow_mut().finish();
                washer.borrow_mut().empty();
            }
            Ok(())
        })
        .map_err(|e| eprintln!("Error in delay future, e: {:?}", e));

    current_thread::spawn(timer);

}

fn stop_laundry(washer: Rc<RefCell<Washer>>) {
    let mut washer = washer.borrow_mut();
    match washer.state() {
        WasherState::Running {..} => {},
        _ => return,
    }
    washer.stop();
}

fn laundry_status() {}
