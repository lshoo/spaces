pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use anyhow::{Ok, Result};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

const HOME: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <title>"HTMX + Spin Example"</title>
</head>
<body>
    <button hx-post="/htmx/clicked" hx-swap="outerHTML">Click Me</button>

    <script src="https://unpkg.com/htmx.org@1.9.4"></script>
</body>

</html>
"#;

#[http_component]
fn handle_htmx(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());

    Ok(http::Response::builder()
        .status(200)
        .header("frontend", "htmx")
        .header("backend", "spin")
        .body(Some(HOME.into()))?
    )
}