use dotenv::dotenv;
use rave_api::options::RaveApiOptions;

mod log;

#[tokio::main]
async fn main() {
    dotenv().ok();

    log::init();
    rave_api::serve(RaveApiOptions::try_from_env().expect("invalid environ")).await;
}
