use axum::{http::StatusCode, service, Router};
use gray_matter::{engine::YAML, Matter, Pod};
use std::{convert::Infallible, fs, net::SocketAddr, path::Path, thread, time::Duration};
use tower_http::services::ServeDir;

mod templates; // how we call in our templates
const CONTENT_DIR: &str = "content"; // relative to the root cargo directory
const PUBLIC_DIR: &str = "news";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    rebuild_site(CONTENT_DIR, PUBLIC_DIR).expect("Error initializing first build");

    tokio::task::spawn_blocking(move || {
        println!("listening for changes: {}", CONTENT_DIR);
        let mut hotwatch = hotwatch::Hotwatch::new().expect("hotwatch failed to initialize!");
        hotwatch
            .watch(CONTENT_DIR, |_| {
                println!("Rebuilding site");
                rebuild_site(CONTENT_DIR, PUBLIC_DIR).expect("Rebuilding site");
            })
            .expect("failed to watch content folder!");
        loop {
            thread::sleep(Duration::from_secs(2)); // we sleep for a time between evals
        }
    });

    // we don't really need all this rendering going forward
    // all we really want is the html
    let app = Router::new().nest(
        "/",
        service::get(ServeDir::new(PUBLIC_DIR)).handle_error(|error: std::io::Error| {
            Ok::<_, Infallible>((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            ))
        }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("serving site on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
fn parse_markdown_file(markdown_file_path: &String) -> (Option<Pod>, String) {
    let markdown: String = fs::read_to_string(&markdown_file_path).unwrap();
    let matter = Matter::<YAML>::new();
    let markdown = matter.parse(&markdown);

    let headers: Option<Pod> = markdown.data;
    let content: String = markdown.content;
    (headers, content)
}

fn rebuild_site(content_dir: &str, output_dir: &str) -> Result<(), anyhow::Error> {
    let _ = fs::remove_dir_all(output_dir);

    let markdown_files: Vec<String> = walkdir::WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .map(|e| e.path().display().to_string())
        .collect();
    let mut html_files = Vec::with_capacity(markdown_files.len());

    let mut iterator = markdown_files.iter().enumerate().peekable();
    let mut previous_file: Option<String> = None;
    loop {
        match iterator.next() {
            Some(current_item) => {
                let current_file = current_item.1;
                let next_file = match iterator.peek() {
                    None => None,
                    Some(_next_file) => Some(
                        iterator
                            .peek()
                            .unwrap()
                            .1
                            .trim_start_matches(content_dir)
                            .replace(".md", ".html"),
                    ),
                };

                let mut html = templates::HEADER.to_owned();
                let (header, markdown) = parse_markdown_file(&current_file);
                // let markdown = fs::read_to_string(&current_file)?;
                let parser =
                    pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all());

                // ok, this is where we parse the markdown

                let mut body = String::new();
                let title = header.as_ref().unwrap()["title"].as_string().unwrap();
                pulldown_cmark::html::push_html(&mut body, parser);
                html.push_str(templates::render_body(&body).as_str());
                html.push_str(templates::render_title(&title).as_str());
                html.push_str(templates::render_footer(previous_file, next_file).as_str());

                let html_file = current_file
                    .replace(content_dir, output_dir)
                    .replace(".md", ".html");
                // set folder == public folder
                let folder = Path::new(&html_file).parent().unwrap();
                let _ = fs::create_dir_all(folder);
                fs::write(&html_file, html)?;
                html_files.push(html_file);

                previous_file = Some(
                    current_file
                        .trim_start_matches(content_dir)
                        .replace(".md", ".html"),
                );
            }
            _ => break,
        }
    }

    write_index(html_files, output_dir)?;
    Ok(())
}

fn write_index(files: Vec<String>, output_dir: &str) -> Result<(), anyhow::Error> {
    let mut html = templates::HEADER.to_owned();
    html.push_str(&templates::render_title("News"));
    let body = files
        .into_iter()
        .map(|file| {
            let file = file.trim_start_matches(output_dir);
            let title = file.trim_start_matches("/").trim_end_matches(".html");
            format!(r#"<a href="{}">{}</a>"#, file, title)
        })
        .collect::<Vec<String>>()
        .join("<br />\n");

    html.push_str(templates::render_body(&body).as_str());
    html.push_str(templates::FOOTER);

    let index_path = Path::new(&output_dir).join("index.html");
    fs::write(index_path, html)?;
    Ok(())
}
