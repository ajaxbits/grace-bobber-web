use std::{fs, path::Path};

mod index_template;
mod templates;
use index_template::generate_index_card;
// how we call in our templates // how we call in our templates
use templates::{Markdown, MarkdownParseError};

const CONTENT_DIR: &str = "news_content"; // relative to the root cargo directory
const PUBLIC_DIR: &str = "news";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    rebuild_site(CONTENT_DIR, PUBLIC_DIR).expect("Error building site");
    Ok(())
}

fn rebuild_site(content_dir: &str, output_dir: &str) -> Result<(), anyhow::Error> {
    let _ = fs::remove_dir_all(output_dir);

    let markdown_files: Result<Vec<Markdown>, MarkdownParseError> =
        walkdir::WalkDir::new(content_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().display().to_string().ends_with(".md"))
            .map(|e| e.path().display().to_string())
            .map(|e| Markdown::new(&e))
            .collect();

    match markdown_files {
        Err(why) => panic!("{:?}", why),
        Ok(mut markdown_files) => {
            markdown_files.sort_by_key(|e| e.date);
            markdown_files.reverse();
            markdown_files.retain(|x| x.file_name != format!("{}/_index.md", CONTENT_DIR));

            let mut html_files = Vec::with_capacity(markdown_files.len());

            let mut iterator = markdown_files.iter().enumerate().peekable();
            let mut next_article: Option<&Markdown> = None;
            loop {
                match iterator.next() {
                    Some(current_item) => {
                        let current_file = current_item.1;
                        let previous_article = match iterator.peek() {
                            None => None,
                            Some(_previous_article) => Some(iterator.peek().unwrap().1),
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

                        let folder = Path::new(&html_file).parent();
                        match folder {
                            Some(parent_folder) => {
                                let _ = fs::create_dir_all(parent_folder);
                                fs::write(&html_file, html)?;
                                html_files.push(html_file);

                                next_article = Some(current_file);
                            }
                            None => {
                                eprintln!("No public folder found. Are you in the right directory? Right now, the public directory is set to: {}", output_dir);
                            }
                        }
                    }
                    _ => break,
                }
            }
            write_index(markdown_files, output_dir)?;
            Ok(())
        }
    }
}

fn write_index(files: Vec<Markdown>, output_dir: &str) -> Result<(), anyhow::Error> {
    let mut index_object = Markdown::new(&format!("{}/_index.md", CONTENT_DIR)).unwrap();
    let body: String = files
        .into_iter()
        .map(|file| generate_index_card(file))
        .collect::<Vec<String>>()
        .join("\n");

    index_object.html_content = body;

    let mut html = templates::render_header(&index_object);
    html.push_str(index_template::render_index_body(index_object).as_str());
    html.push_str(templates::FOOTER);

    let index_path = Path::new(&output_dir).join("index.html");
    fs::write(index_path, html)?;
    Ok(())
}
