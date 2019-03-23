use labo;

fn main() {
    let config = match labo::Config::new() {
        Ok(config) => config,
        Err(e) => panic!(e),
    };

    labo::run_bot(config);

}
