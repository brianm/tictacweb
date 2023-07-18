#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

use rocket::response::Redirect;
use rocket::State;
use dashmap::DashMap;
use rocket::response::status::NotFound;
use ulid::Ulid;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/game/<game_id>")]
fn game(state: &State<World>, game_id: &str) -> Result<String, NotFound<String>> {
    let go = state.games.get(game_id);
    match go {
        Some(game) => return Ok(format!("{:?}", game.value())),
        None => return Err(NotFound(format!("Game {} not found", game_id))),
    };
}

#[post("/new")]
fn new_game(state: &State<World>) -> Redirect {
    let game_id = Ulid::new().to_string();
    state.games.insert(game_id.clone(), GameState::NotStarted);
    Redirect::to(uri!(game(game_id = game_id)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(World { 
        games: DashMap::new()
    })
    .mount("/", routes![index, new_game, game])
}

#[derive(Debug, Serialize, Deserialize)]
enum GameState {
    NotStarted,
    XTurn,
    YTurn,
    XWon,
    YWon,
}

struct World {
    games: DashMap<String, GameState>,
}