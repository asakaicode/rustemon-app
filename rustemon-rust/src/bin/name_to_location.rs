use super::translate_name::Region;
use rustemon::{client::RustemonClient, model::locations::Location};

async fn process_data(location: &Location) {
    println!("{:?}", location);
}

#[tokio::main]
async fn main() {
    let rustemon_client = RustemonClient::default();
    let location =
        rustemon::locations::location::get_by_name("postwick-town", &rustemon_client).await;
    match location {
        Ok(l) => process_data(&l).await,
        Err(err) => println!("{}", err),
    }
}
