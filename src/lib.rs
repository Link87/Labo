use std::env;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Config {
    pub api_token: String,
    programs: Vec<Program>,
}

impl Config {
    pub fn new() -> Result<Config, ()> {
        let api_token = match env::var("TELEGRAM_BOT_TOKEN") {
            Ok(token) => token,
            Err(_) => return Err(()),
        };
        Ok(Config {
            api_token,
            programs: Vec::new(),
        })
    }
}

struct Washer {

}

#[derive(PartialEq, Debug)]
struct Program {
    name: String,
    temp: u8,
    duration: u16,
}