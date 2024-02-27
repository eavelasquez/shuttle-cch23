use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn internal_error() -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
}

async fn cube_the_bits(Path(params): Path<String>) -> String {
    let packet_ids: Vec<u32> = params
        .split('/')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    let xor_result = packet_ids.into_iter().fold(0, |acc, x| acc ^ x);
    format!("{}", xor_result.pow(3))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(internal_error))
        .route("/1/*packet_ids", get(cube_the_bits));

    Ok(router.into())
}
