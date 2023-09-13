use crate::{Context, Response};
use hyper::StatusCode;
use serde::Deserialize;
use xin::chat::*;
use xin::completions::*;
use xin::embeddings::*;

pub async fn test_handler(ctx: Context) -> String {
    format!("test called, state_thing was: {}", ctx.state.state_thing)
}

#[derive(Deserialize)]
struct SendRequest {
    name: String,
    active: bool,
}

pub async fn send_handler(mut ctx: Context) -> Response {
    let body: SendRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap()
        }
    };

    Response::new(
        format!(
            "send called with name: {} and active: {}",
            body.name, body.active
        )
        .into(),
    )
}

pub async fn param_handler(ctx: Context) -> String {
    let param = match ctx.params.find("some_param") {
        Some(v) => v,
        None => "empty",
    };
    format!("param called, param was: {}", param)
}

pub async fn chat_completions_handler(mut ctx: Context) -> String {
    let body: CreateChatCompletionRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            panic!("could not parse JSON: {}", e)

            // return hyper::Response::builder()
            //     .status(StatusCode::BAD_REQUEST)
            //     .body(format!("could not parse JSON: {}", e).into())
            //     .unwrap()
        }
    };

    dbg!(&body);

    // let _response = Response::new(
    //     format!(
    //         "send called with name: {} and active: {}",
    //         body.name, body.active
    //     )
    //     .into(),
    // );

    format!("chat completions called")
}

pub async fn completions_handler(mut ctx: Context) -> String {
    let body: CreateCompletionRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            panic!("could not parse JSON: {}", e)

            // return hyper::Response::builder()
            //     .status(StatusCode::BAD_REQUEST)
            //     .body(format!("could not parse JSON: {}", e).into())
            //     .unwrap()
        }
    };

    dbg!(&body);

    format!("completions called")
}

pub async fn embeddings_handler(mut ctx: Context) -> String {
    let body: CreateEmbeddingsRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            panic!("could not parse JSON: {}", e)

            // return hyper::Response::builder()
            //     .status(StatusCode::BAD_REQUEST)
            //     .body(format!("could not parse JSON: {}", e).into())
            //     .unwrap()
        }
    };

    dbg!(&body);

    format!("embeddings called")
}
