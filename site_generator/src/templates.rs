pub const HEADER: &str = r#"<!DOCTYPE html>
<html lang="en">

  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
  </head>

  <title>Grace Bobber</title>

"#;

pub fn render_body(body: &str) -> String {
    format!(
        r#"  <body>
    <nav>
        <a href="/">Home</a>
    </nav>
    <br />
    {}
  </body>"#,
        body
    )
}

pub fn render_footer(previous_file: Option<String>, next_file: Option<String>) -> String {
    match (previous_file.clone(), next_file.clone()) {
        (None, Some(..)) => {
            format!(
                r#"  <div>
                    <a href="{}">next_file</a>
                </div></html>"#,
                next_file.unwrap()
            )
        }
        (Some(..), None) => {
            format!(
                r#"  <div>
                    <a href="{}">previous_file</a>
                </div></html>"#,
                previous_file.unwrap()
            )
        }
        (Some(..), Some(..)) => {
            format!(
                r#"  <div>
                    <a href="{}">previous_file</a>
                    <a href="{}">next_file</a>
                </div></html>"#,
                previous_file.unwrap(),
                next_file.unwrap()
            )
        }
        _ => "ERROR LMAO".to_string(),
    }
}

pub const FOOTER: &str = r#"

</html>
"#;
