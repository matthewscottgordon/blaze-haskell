use {
    axum::{
        Json, Router,
        routing::{get, post},
    },
    log::info,
    serde::{Deserialize, Serialize},
    tower_http::trace::TraceLayer,
    tracing_subscriber::EnvFilter,
};

mod api_types;
use api_types::{Battlesnake, Board, Game};

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

async fn post_move(request: Json<MoveRequest>) -> Json<MoveResponse> {
    info!("Receive request for turn {} move.", request.turn);
    Json(MoveResponse {
        move_direction: MoveDirection::Down,
        shout: None,
    })
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("blaze_haskell=debug,tower_http=info"))
                .unwrap(),
        )
        .init();
    let app = Router::new()
        .route("/", get(describe_snake))
        .route("/start", post(post_start))
        .route("/move", post(post_move))
        .route("/end", post(post_end))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
