use crate::{Context, Response};
use hyper::StatusCode;
use serde::Deserialize;
use xin::chat::*;
use xin::completions::*;
use xin::embeddings::*;

const URL_CHAT_COMPLETIONS: &str = "https://api.openai.com/v1/chat/completions";

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
    let body: ChatRequest = match ctx.body_json().await {
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

    let s = serde_json::to_string(&body).unwrap();
    dbg!(&s);

    // openai::Client::new(std::env::var("OPENAI_API_KEY").unwrap().to_string())
    //     .chat_completion(body)
    //     .await
    //     .unwrap();

    {
        let conn = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();

        let client = hyper::Client::builder().build::<_, hyper::Body>(conn);

        let uri = URL_CHAT_COMPLETIONS.parse::<hyper::Uri>().unwrap();

        // create request
        let request = hyper::Request::builder()
            .uri(uri)
            .method("POST")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!(
                    "Bearer {openai_api_key}",
                    openai_api_key = std::env::var("OPENAI_API_KEY").unwrap()
                ),
            )
            .body(hyper::Body::from(serde_json::to_string(&body).unwrap()))
            .unwrap();

        // send request
        let res = client.request(request).await;
        dbg!(&res);
    }
    // send request to openai
    {
        let auth = format!(
            "Authorization: Bearer {openai_api_key}",
            openai_api_key = std::env::var("OPENAI_API_KEY").unwrap()
        );
        let status = std::process::Command::new("curl")
            .args([
                "https://api.openai.com/v1/chat/completions",
                "-X",
                "POST",
                "-H",
                "Content-Type: application/json",
                "-H",
                &auth,
                "-d",
                &s,
                // r###"{
                //     "model": "gpt-3.5-turbo",
                //     "messages": [
                //       {
                //         "role": "system",
                //         "content": "You are a helpful assistant."
                //       },
                //       {
                //         "role": "user",
                //         "content": "Hello!"
                //       }
                //     ]
                //   }"###,
            ])
            .status()
            .unwrap();

        // let output = std::process::Command::new("curl")
        //     .arg("https://api.openai.com/v1/models")
        //     .arg("-H")
        //     .arg("Authorization: Bearer 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")
        //     .output()
        //     .unwrap();
        // dbg!(&output);
    }

    // let _response = Response::new(
    //     format!(
    //         "send called with name: {} and active: {}",
    //         body.name, body.active
    //     )
    //     .into(),
    // );

    format!("chat completions called")
}

fn create_chat_request() -> ChatRequest {
    let model = "gpt-4";
    // create messages
    let mut messages: Vec<ChatRequestMessage> = vec![];
    // messages.push(ChatRequestMessage {
    //     role: ChatRequestRole::System,
    //     content: String::from("You are a helpfule assistant."),
    //     name: None,
    //     function_call: None,
    // });
    messages.push(ChatRequestMessage {
        role: ChatRequestRole::User,
        content: String::from("What is Bitcoin?"),
        name: None,
        function_call: None,
    });
    // let sampling = ChatRequestSampling::Temperature(0.8);

    ChatRequestBuilder::new(model, messages)
        // .with_sampling(sampling)
        .build()
}

pub async fn completions_handler(mut ctx: Context) -> String {
    let body: CompletionRequest = match ctx.body_json().await {
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
    let s = serde_json::to_string(&body).unwrap();
    dbg!(&s);

    let auth = format!(
        "Authorization: Bearer {openai_api_key}",
        openai_api_key = std::env::var("OPENAI_API_KEY").unwrap()
    );

    let status = std::process::Command::new("curl")
        .args([
            "https://api.openai.com/v1/completions",
            "-H",
            "Content-Type: application/json",
            "-H",
            &auth,
            "-d",
            &s,
            // r###"{
            //     "model": "text-davinci-003",
            //     "prompt": "Say this is a test",
            //     "max_tokens": 7,
            //     "temperature": 0
            //   }"###,
            // r###"{
            //     "model": "gpt-3.5-turbo",
            //     "messages": [
            //       {
            //         "role": "system",
            //         "content": "You are a helpful assistant."
            //       },
            //       {
            //         "role": "user",
            //         "content": "Hello!"
            //       }
            //     ]
            //   }"###,
        ])
        .status()
        .unwrap();

    format!("completions called")
}

pub async fn embeddings_handler(mut ctx: Context) -> String {
    let body: EmbeddingsRequest = match ctx.body_json().await {
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

mod openai {
    use super::error::APIError;
    use hyper::{client::connect::HttpConnector, Body, Request, Uri};
    use xin::chat::*;

    const API_URL_V1: &str = "https://api.openai.com/v1";
    const URL_CHAT_COMPLETIONS: &str = "https://api.openai.com/v1/chat/completions";

    pub struct Client {
        pub api_endpoint: String,
        pub api_key: String,
        pub organization: Option<String>,
        pub client: hyper::Client<HttpConnector, Body>,
    }

    impl Client {
        pub fn new(api_key: String) -> Self {
            let endpoint =
                std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned());
            Self::new_with_endpoint(endpoint, api_key)
        }

        pub fn new_with_endpoint(api_endpoint: String, api_key: String) -> Self {
            Self {
                api_endpoint,
                api_key,
                organization: None,
                client: hyper::Client::new(),
            }
        }

        pub fn new_with_organization(api_key: String, organization: String) -> Self {
            let endpoint =
                std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| API_URL_V1.to_owned());
            Self {
                api_endpoint: endpoint,
                api_key,
                organization: organization.into(),
                client: hyper::Client::new(),
            }
        }

        // pub fn build_request_old(&self, request: minreq::Request) -> minreq::Request {
        //     let mut request = request
        //         .with_header("Content-Type", "application/json")
        //         .with_header("Authorization", format!("Bearer {}", self.api_key));
        //     if let Some(organization) = &self.organization {
        //         request = request.with_header("openai-organization", organization);
        //     }
        //     request
        // }

        pub async fn post<T: serde::ser::Serialize>(
            &self,
            path: &str,
            data: &T,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let url = format!(
                "{api_endpoint}{path}",
                api_endpoint = self.api_endpoint,
                path = path
            );
            let uri = url.parse::<Uri>()?;

            let uri = URL_CHAT_COMPLETIONS.parse::<Uri>()?;

            dbg!(&uri);

            dbg!("build request");

            // create request
            let request = Request::builder()
                .method("POST")
                .uri(uri)
                .header("CONTENT_TYPE", "application/json")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .body(Body::from(serde_json::to_string(&data)?))?;

            dbg!("send request");

            let res = self.client.request(request).await;
            dbg!(&res);

            dbg!("got response");

            // let status = response.status();
            // let body = hyper::body::to_bytes(response.into_body()).await?;

            // println!("Status: {}", status);
            // println!("Body: {:?}", body);

            // let res = request.with_json(params).unwrap().send();
            // match res {
            //     Ok(res) => {
            //         if (200..=299).contains(&res.status_code) {
            //             Ok(res)
            //         } else {
            //             Err(APIError {
            //                 message: format!("{}: {}", res.status_code, res.as_str().unwrap()),
            //             })
            //         }
            //     }
            //     Err(e) => Err(self.new_error(e)),
            // }

            Ok(())
        }

        pub async fn chat_completion(
            &self,
            req: ChatRequest,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let res = self.post("/chat/completions", &req).await?;
            // let r = res.json::<ChatResponse>();
            Ok(())
        }
    }
}

pub mod error {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub struct APIError {
        pub message: String,
    }

    impl fmt::Display for APIError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "APIError: {}", self.message)
        }
    }

    impl Error for APIError {}
}
