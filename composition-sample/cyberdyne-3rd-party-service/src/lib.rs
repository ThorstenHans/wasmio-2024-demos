use serde::Deserialize;
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;
mod bindings;
use bindings::fermyon::hmac::verify::verify;
use spin_sdk::sqlite::{Connection, Value};

#[derive(Debug, Deserialize)]
pub struct HandshakeRequestModel {
    #[serde(rename = "keyData")]
    key_data: String,
}
/// A simple Spin HTTP component.
#[http_component]
fn handle_consumer(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.post("/target", handle_invocation);
    Ok(router.handle(req))
}

fn handle_invocation(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let r = match is_handshake(&req) {
        true => handle_hanshake(&req, params),
        false => handle_webhook_invocation(&req, params),
    };
    Ok(r)
}

fn is_handshake(req: &Request) -> bool {
    let hs = req.query().contains("handshake=true");
    match hs {
        true => {
            println!("Handling web hook verification request...");
            hs
        }
        false => {
            println!("Handling web hook invocation...");
            hs
        }
    }
}

fn handle_hanshake(req: &Request, _: Params) -> anyhow::Result<Response> {
    let Ok(model) = serde_json::from_slice::<HandshakeRequestModel>(req.body()) else {
        println!("Could not deserialize payload");
        return Ok(Response::builder().status(500).body(()).build());
    };
    let con = Connection::open_default()?;
    let parameters = [Value::Text(model.key_data)];
    match con.execute("INSERT INTO KEYS (KEY) VALUES (?)", &parameters) {
        Ok(_) => Ok(Response::builder().status(200).body(()).build()),
        Err(e) => {
            println!("Error while inserting data {:?}", e);
            Ok(Response::builder().status(500).body(()).build())
        }
    }
}

fn handle_webhook_invocation(req: &Request, _: Params) -> anyhow::Result<Response> {
    let con = Connection::open_default()?;
    let rowset = con.execute("SELECT KEY FROM KEYS", &[])?;
    let Some(found) = rowset.rows().next() else {
        return Ok(Response::builder().status(500).body(()).build());
    };
    let key_data = found.get::<&str>("KEY").unwrap();

    let body = req.body();
    let tag = req.header("X-Signature").unwrap().as_bytes();
    //let tag = HEXUPPER.decode(tag.as_bytes())?;
    match verify(body, key_data.as_bytes(), tag) {
        true => {
            println!("Payload signature is VALID.");
            Ok(Response::builder().status(200).body(()).build())
        }
        false => {
            println!("Payload signature is INVALID.");
            Ok(Response::builder().status(500).body(()).build())
        }
    }
}
