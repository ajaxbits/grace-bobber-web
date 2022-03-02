use std::{fs, path::Path};

mod templates; // how we call in our templates
use templates::Markdown;

const CONTENT_DIR: &str = "news_content"; // relative to the root cargo directory
const PUBLIC_DIR: &str = "news";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    rebuild_site(CONTENT_DIR, PUBLIC_DIR).expect("Error building site");
    Ok(())
}

fn rebuild_site(content_dir: &str, output_dir: &str) -> Result<(), anyhow::Error> {
    let _ = fs::remove_dir_all(output_dir);

    let markdown_files: Vec<Markdown> = walkdir::WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .map(|e| e.path().display().to_string())
        .map(|e| Markdown::new(&e))
        .collect();

    let mut html_files = Vec::with_capacity(markdown_files.len());

    let mut iterator = markdown_files.iter().enumerate().peekable();
    let mut next_article: Option<&Markdown> = None;
    loop {
        match iterator.next() {
            Some(current_item) => {
                let current_file = current_item.1;
                let previous_article = match iterator.peek() {
                    None => None,
                    Some(_next_file) => Some(iterator.peek().unwrap().1),
                };

                let mut html = String::new();
                html.push_str(templates::render_header(&current_file).as_str());
                html.push_str(templates::render_body(&current_file).as_str());
                html.push_str(
                    templates::render_bottom_navigation(
                        next_article,
                        previous_article,
                        content_dir,
                    )
                    .as_str(),
                );
                html.push_str(templates::FOOTER);

                let html_file = current_file
                    .file_name
                    .replace(content_dir, output_dir)
                    .replace(".md", ".html");
                // set folder == public folder
                let folder = Path::new(&html_file).parent().unwrap();
                let _ = fs::create_dir_all(folder);
                fs::write(&html_file, html)?;
                html_files.push(html_file);

                next_article = Some(current_file);
            }
            _ => break,
        }
    }

    write_index(html_files, output_dir)?;
    Ok(())
}

fn write_index(files: Vec<String>, output_dir: &str) -> Result<(), anyhow::Error> {
    // TODO make cleaner
    let mut index_object = Markdown {
        file_name: "".to_owned(),
        title: "News".to_owned(),
        date: chrono::NaiveDate::from_ymd(2000, 1, 1),
        image: "".to_owned(),
        markdown_content: "".to_owned(),
        html_content: "".to_owned(),
    };
    let mut html = templates::render_header(&index_object);
    let body = files
        .into_iter()
        .map(|file| {
            let file = format!("/news{}", file.trim_start_matches(output_dir));
            // TODO fix this
            let title = file.trim_start_matches("/").trim_end_matches(".html");
            format!(r#"<a href="{}">{}</a>"#, file, title)
        })
        .collect::<Vec<String>>()
        .join("<br />\n");

    index_object.html_content = body;
    html.push_str(templates::render_body(&index_object).as_str());
    html.push_str(templates::FOOTER);

    let index_path = Path::new(&output_dir).join("index.html");
    fs::write(index_path, html)?;
    Ok(())
}
