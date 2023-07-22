#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

use dashmap::DashMap;
use rocket::response::status::NotFound;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use ulid::Ulid;

#[post("/game")]
fn new_game(state: &State<World>) -> Redirect {
    let game_id = Ulid::new().to_string();
    state.games.insert(
        game_id.clone(),
        GameState::NotStarted {
            board: Board([[' '; 3]; 3]),
        },
    );
    Redirect::to(uri!(game(game_id = game_id)))
}

#[get("/game/<game_id>")]
fn game(state: &State<World>, game_id: &str) -> Result<Json<GameState>, NotFound<String>> {
    let go = state.games.get(game_id);
    match go {
        Some(game) => return Ok(Json::from(game.value().clone())),
        None => return Err(NotFound(format!("Game {} not found", game_id))),
    };
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(World {
            games: DashMap::new(),
        })
        .mount("/", routes![index, new_game, game])
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
enum GameState {
    NotStarted { board: Board },
    XTurn { board: Board },
    YTurn { board: Board },
    XWon { board: Board },
    YWon { board: Board },
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
struct Board([[char; 3]; 3]);

struct World {
    games: DashMap<String, GameState>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}