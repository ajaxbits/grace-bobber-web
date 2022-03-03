use std::path::Path;

use crate::{templates::Markdown, CONTENT_DIR};

pub fn render_index_body(index_object: Markdown) -> String {
    format!(
        r#"
			<main class="content">
				<div class="cardList">
                {}
				</div>
			</main>
		</section> "#,
        index_object.html_content
    )
}

pub fn generate_index_card(article: Markdown) -> String {
    format!(
        r#"
					<a href="{}" class="card">
						<img
							class="card-image"
							src="{}"
							alt="{}"
                            id="{}"
						/>
						<p class="card-date small-caps">{}</p>
						<h4 class="card-subtitle">{}</h4>
					</a>

                       "#,
        article
            .file_name
            .trim_start_matches(CONTENT_DIR)
            .trim_start_matches("/")
            .replace(".md", ".html"),
        article.image,
        Path::new(&article.image)
            .file_stem()
            .unwrap()
            .to_string_lossy(),
        Path::new(&article.image)
            .file_stem()
            .unwrap()
            .to_string_lossy(),
        article.date.format("%B %e, %Y"),
        article.title
    )
}
