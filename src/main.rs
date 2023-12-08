mod utils;
pub mod config;
use utils::show_logo;


fn main() {
    show_logo();
    let config = config::load_config();
    match config {
        Ok(config) => println!("{:?}", config),
        Err(err) => config::create_default_config().unwrap() ,
    }
}
