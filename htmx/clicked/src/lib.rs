use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

const CLICKED: &str = r###"
<div id="msg-container">
    <p>"Yay! You clicked the button!"</p>
    <button hx-post="/htmx/gohome" hx-swap="outerHTML" hx-target="#msg-container">Restore original state</button>
</div>
"###;

#[http_component]
fn handle_clicked(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .header("key", "value")
        .body(Some(CLICKED.into()))?)
}