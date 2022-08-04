use rustemon::{client::RustemonClient, model::locations::Location, locations::location};

fn process_data(location: &Location) {
    let mut game_indices = location.game_indices.unwrap();
    let game_indice = game_indices.pop().unwrap();
    println!("{:?}", game_indice);
}

#[tokio::main]
async fn main() {
    let rustemon_client = RustemonClient::default();
    let location =
        location::get_by_name("cerulean-city", &rustemon_client).await;
    match location {
        Ok(l) => process_data(&l),
        Err(err) => println!("{}", err),
    }
}
