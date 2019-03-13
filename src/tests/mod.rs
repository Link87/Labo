use std::env;

use super::{ Config, Program };

#[test]
fn config_load_api_token() {
    let old = env::var("TELEGRAM_BOT_TOKEN");
    env::set_var("TELEGRAM_BOT_TOKEN", "token_test_blah");
    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => panic!(e),
    };
    assert_eq!(config.api_token, "token_test_blah");

    match old {
        Ok(var) => env::set_var("TELEGRAM_BOT_TOKEN", var),
        _ => (),
    }
}

#[test]
fn config_deserialize_programs() {
    let programs = vec![
        Program { name: String::from("Baumwolle"), temp: 30, duration: 200 },
        Program { name: String::from("Baumwolle"), temp: 40, duration: 210 },
        Program { name: String::from("Baumwolle"), temp: 60, duration: 250 },
        Program { name: String::from("Plegeleicht"), temp: 30, duration: 150 },
    ];

    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => panic!(e),
    };

    assert_eq!(config.programs.len(), programs.len());
    for i in 1..programs.len() {
        assert_eq!(programs[i], config.programs[i]);
    }
}