use {
    axum::{
        Json, Router,
        routing::{get, post},
    },
    log::info,
    serde::{Deserialize, Serialize},
};

use blaze_haskell::{
    api_types::{Battlesnake, Board, Game},
    error::Result,
    game_state,
    game_state::GameState,
    planner,
};

static BATTLESNAKE_API_VERSION: &str = "1";
static AUTHOR: &str = "Matt Gordon";
static SNAKE_VERSION: &str = "pre-alpha";
static SNAKE_COLOR: &str = "#00ff00";
static SNAKE_HEAD: &str = "default";
static SNAKE_TAIL: &str = "default";

#[derive(Serialize)]
struct SnakeDetails {
    apiversion: String,
    author: Option<String>,
    color: Option<String>,
    head: Option<String>,
    tail: Option<String>,
    version: Option<String>,
}

async fn describe_snake() -> Json<SnakeDetails> {
    let apiversion = BATTLESNAKE_API_VERSION.into();
    let author = Some(AUTHOR.into());
    let color = Some(SNAKE_COLOR.into());
    let head = Some(SNAKE_HEAD.into());
    let tail = Some(SNAKE_TAIL.into());
    let version = Some(SNAKE_VERSION.into());
    Json(SnakeDetails {
        apiversion,
        author,
        color,
        head,
        tail,
        version,
    })
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct StartRequest {
    game: Game,
    turn: i32,
    board: Board,
    you: Battlesnake,
}

async fn post_start(request: Json<StartRequest>) -> () {
    info!("Started new game with id{}.", request.game.id);
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
#[allow(dead_code)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct MoveRequest {
    game: Game,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

#[derive(Serialize)]
struct MoveResponse {
    #[serde(rename = "move")]
    move_direction: MoveDirection,
    shout: Option<String>,
}

impl From<game_state::Move> for MoveResponse {
    fn from(value: game_state::Move) -> Self {
        match value {
            game_state::Move::Up => MoveResponse {
                move_direction: MoveDirection::Up,
                shout: None,
            },
            game_state::Move::Down => MoveResponse {
                move_direction: MoveDirection::Down,
                shout: None,
            },
            game_state::Move::Left => MoveResponse {
                move_direction: MoveDirection::Left,
                shout: None,
            },
            game_state::Move::Right => MoveResponse {
                move_direction: MoveDirection::Right,
                shout: None,
            },
        }
    }
}

async fn post_move(request: Json<MoveRequest>) -> Result<Json<MoveResponse>> {
    info!("Receive request for turn {} move.", request.turn);
    info!("Latency on last turn was {}.", request.you.latency);
    Ok(Json(
        planner::devise_plan(GameState::from_board(&request.board, &request.you.id)?)
            .await
            .into(),
    ))
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct EndRequest {
    game: Game,
    turn: i32,
    board: Board,
    you: Battlesnake,
}

async fn post_end(body: Json<EndRequest>) -> () {
    info!("Game with id {} ended.", body.game.id)
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(describe_snake))
        .route("/start", post(post_start))
        .route("/move", post(post_move))
        .route("/end", post(post_end))
}
