use telegram_bot_fork::User;
use telegram_bot_fork::*;
use tokio::prelude::*;
use tokio::runtime::current_thread;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::washer::*;

#[derive(Debug)]
pub struct Client {
    user: Rc<User>,
    state: ClientState,
    washer: Rc<RefCell<Washer>>,
}

#[derive(Debug)]
enum ClientState {
    Nothing,
    ReceivedStart { program: Option<Program> },
}

impl Client {
    pub fn new(user: User, washer: Rc<RefCell<Washer>>) -> Client {
        Client {
            user: Rc::new(user),
            state: ClientState::Nothing,
            washer,
        }
    }

    /// Analyzes the given message and calls code according to its content.
    /// Messages not being text messages are ignored.
    pub fn handle_message(&self, api: Api, message: Message) {
        if let MessageKind::Text {
            ref data,
            ref entities,
        } = message.kind
        {
            // Print received text message to stdout.
            println!("<{}>: {}", &message.from.first_name, data);

            // test if message contains command
            if let Some(cmd_entity) = entities
                .iter()
                .find(|entity| entity.kind == MessageEntityKind::BotCommand)
            {
                let cmd: String = data
                    .chars()
                    .skip(cmd_entity.offset as usize)
                    .take(cmd_entity.length as usize)
                    .collect();
                println!("Received command: {}", cmd);
                match cmd.as_ref() {
                    "/start" => self.handle_start_command(
                        api,
                        data.chars()
                            .skip((cmd_entity.offset + cmd_entity.length) as usize)
                            .collect(),
                    ),
                    "/stop" => self.handle_stop_command(api),
                    "/status" => self.handle_status_command(api),
                    _ => {}
                }
            } else {
                api.spawn(self.user.text(format!(
                    "Du hast '{}' geschrieben. Leider habe ich das nicht verstanden.",
                    data
                )));
            }
        }
    }

    fn handle_start_command(&self, api: Api, text: String) {
        match &self.state {
            ClientState::Nothing => {}
            ClientState::ReceivedStart { .. } => {}
        }

        match self.washer.borrow_mut().state() {
            WasherState::Idle => {}
            _ => return,
        }

        let program: Program;
        match text.trim() {
            "Baumwolle 30" => {
                program = Program::new(String::from("Baumwolle 30°C"), Duration::from_secs(5))
            }
            _ => return,
        }

        self.start_laundry(api, program);
    }

    fn handle_stop_command(&self, api: Api) {
        let mut washer = self.washer.borrow_mut();
        match washer.state() {
            WasherState::Running { .. } => {
                washer.stop();
                api.spawn(self.user.text(
                    "Der Waschvorgang wurde abgebrochen. 
Du wirst nun nicht mehr benachrichtigt.",
                ));
            }
            WasherState::Finished => return,
            WasherState::Idle => return,
        }
    }

    fn handle_status_command(&self, api: Api) {
        let mut washer = self.washer.borrow_mut();
        match washer.state() {
            WasherState::Running { user, .. } if user.0 == self.user.id.0 => {
                if let Some(remaining) = washer.remaining_time() {
                    api.spawn(self.user.text(format!(
                        "Deine Wäsche braucht noch {} Minuten.",
                        remaining.as_secs() /* / 60*/
                    )));
                } else {
                    washer.finish();
                }
            }
            WasherState::Running { user, .. } => {
                if let Some(remaining) = washer.remaining_time() {
                    api.spawn(self.user.text(format!(
                        "{} hat gerade Wäsche in der Maschine.
Diese ist in {} Minuten fertig.",
                        user,
                        remaining.as_secs() /* / 60*/
                    )));
                } else {
                    washer.finish();
                }
            }
            WasherState::Finished => {}
            WasherState::Idle => {
                api.spawn(self.user.text(
                    "Die Waschmaschine läuft gerade nicht.
Starte ein Programm mit /start.",
                ));
            }
        }
    }

    /// Starts the `Timer`.
    fn start_laundry(&self, api: Api, program: Program) {
        let washer = Rc::clone(&self.washer);
        let user = Rc::clone(&self.user);
        let timer = washer.borrow_mut().start(&program, self.user.id);
        let timer = timer
            .and_then(move |succeeded| {
                if succeeded {
                    api.spawn(user.text("Your laundry is done"));
                    washer.borrow_mut().finish();
                    washer.borrow_mut().empty();
                }
                Ok(())
            })
            .map_err(|e| eprintln!("Error in delay future, e: {:?}", e));

        current_thread::spawn(timer);
    }
}
