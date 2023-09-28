use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

// restore original button from the homepage...
const RESTORE: &str = r#"
    <button hx-post="/htmx/clicked" hx-swap="outerHTML">
        Click Me
    </button>
"#;

#[http_component]
fn handle_go_home(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .body(Some(RESTORE.into()))?)
}