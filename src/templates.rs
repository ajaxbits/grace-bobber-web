use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Clone)]
pub struct Markdown {
    pub file_name: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub image: String,
    pub markdown_content: String,
    pub html_content: String,
}

#[derive(Debug, Clone, Deserialize)]
struct FrontMatter {
    title: String,
    date: chrono::NaiveDate,
    image: String,
}
#[derive(Debug)]
pub enum MarkdownParseError {
    TitleError,
    DateError,
    EmageError,
    FrontMatterError,
    GenericParseError,
}

impl Markdown {
    pub fn new(markdown_file_path: &String) -> Result<Self, MarkdownParseError> {
        let markdown: String = fs::read_to_string(&markdown_file_path).unwrap();
        let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();

        let markdown = matter.parse_with_struct::<FrontMatter>(&markdown);

        match markdown {
            Some(markdown) => {
                let headers: FrontMatter = markdown.data;
                let markdown_content = markdown.content;

                let parser = pulldown_cmark::Parser::new_ext(
                    &markdown_content,
                    pulldown_cmark::Options::all(),
                );
                let mut html_content = String::new();
                pulldown_cmark::html::push_html(&mut html_content, parser);

                Ok(Self {
                    file_name: markdown_file_path.to_owned(),
                    title: headers.title,
                    date: headers.date,
                    image: headers.image,
                    markdown_content,
                    html_content,
                })
            }
            None => Err(MarkdownParseError::GenericParseError),
        }
    }
}

pub fn render_header(markdown: &Markdown) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
	<head>
		<meta charset="UTF-8" />
		<meta
			name="viewport"
			content="width=device-width, initial-scale=1.0, viewport-fit=cover"
		/>
        {}
		<meta
			name="description"
			content="Grace Bobber is an actor, musician, and dancer living in Chicago. She graduated from Northwestern University and is represented by Shirley Hamilton Talent."
		/>

		<!-- Open Graph / Facebook -->
		<meta property="og:type" content="website" />
		<meta property="og:url" content="https://www.gracebobber.com/" />
		<meta property="og:title" content="Grace Bobber" />
		<meta
			property="og:description"
			content="Grace Bobber is an actor, musician, and dancer living in Chicago. She graduated from Northwestern University and is represented by Shirley Hamilton Talent."
		/>
		<meta property="og:image" content="../images/headshot.jpg" />

		<!-- Twitter -->
		<meta property="twitter:card" content="summary_large_image" />
		<meta property="twitter:url" content="https://www.gracebobber.com/" />
		<meta property="twitter:title" content="Grace Bobber" />
		<meta
			property="twitter:description"
			content="Grace Bobber is an actor, musician, and dancer living in Chicago. She graduated from Northwestern University and is represented by Shirley Hamilton Talent."
		/>
		<meta property="twitter:image" content="../images/headshot.jpg" />
		<link
			rel="shortcut icon"
			href="/images/sunflower.png"
			type="image/x-icon"
		/>

		<link rel="preload" href="../dist/styles/base.css" as="style" />
		<link rel="preload" href="../dist/styles/global.css" as="style" />
		<link rel="preload" href="../dist/styles/news.css" as="style" />

		<link rel="preconnect" href="https://fonts.gstatic.com/" crossorigin />
		<link
			href="https://fonts.googleapis.com/css2?family=Baskervville:ital@0;1&family=Work+Sans:ital,wght@0,400;0,450;1,400;1,450&display=swap"
			rel="stylesheet"
		/>
		<link rel="stylesheet" href="../dist/styles/base.css" />
		<link rel="stylesheet" href="../dist/styles/global.css" />
		<link rel="stylesheet" href="../dist/styles/news.css" />
		<link rel="preload" href="../dist/scripts/menu.js" as="script" />
	</head>
	<body>
		<section class="contentWrapper">
			<nav class="siteNav">
				<a class="siteNav-siteTitle" href="/">
					<h3>Grace Bobber</h3>
				</a>
				<ul class="menuList">
					<li>
						<a class="menuList-link" href="/#bio">Home</a>
					</li>
					<li>
						<a class="menuList-link" href="/pages/news">News</a>
					</li>
					<li>
						<a class="menuList-link" href="">Media</a>
						<div class="menuList-dropdown">
							<a href="/pages/photos" class="small-caps">Photos</a>
							<a href="/pages/videos" class="small-caps">Videos</a>
						</div>
					</li>
					<li>
						<a class="menuList-link" href="/pages/headshotsresume/"
							>Headshots & Resume</a
						>
					</li>
					<li>
						<a class="menuList-link" href="">Music</a>
						<div class="menuList-dropdown">
							<a href="/pages/music/solo.html" class="small-caps">Solo Work</a>
							<a href="/pages/music/fishercats.html" class="small-caps"
								>The Fisher Cats</a
							>
							<a href="/pages/music/thunk.html" class="small-caps">Thunk</a>
						</div>
					</li>
					<li>
						<a class="menuList-link" href="/pages/contact">Contact</a>
					</li>
				</ul>
				<svg
					class="siteNav-hamburger"
					role="img"
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					aria-labelledby="hamburgerIconTitle"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<title id="hamburgerIconTitle">Menu</title>
					<path d="M6 7L18 7M6 12L18 12M6 17L18 17" />
				</svg>
			</nav>
			<div class="swipeOver">
				<div
					class="swipeOver-linkListWrapper"
					style="display: flex;justify-content: space-around;width: 100%;"
				>
					<ul class="swipeOver-linkList">
						<li>
							<a class="swipeOver-link" href="/#bio">Home</a>
						</li>
						<li>
							<a class="swipeOver-link" href="/pages/news">News</a>
						</li>
						<li>
							<a class="swipeOver-link" href="/#">Media</a>
							<div class="swipeOver-dropdown">
								<a href="/pages/photos" class="small-caps">Photos</a>
								<a href="/pages/videos" class="small-caps">Videos</a>
							</div>
						</li>
						<li>
							<a class="swipeOver-link" href="/pages/headshotsresume/"
								>Headshots & Resume</a
							>
						</li>
						<li>
							<a class="swipeOver-link" href="/#">Music</a>
							<div class="swipeOver-dropdown">
								<a href="/pages/music/solo.html" class="small-caps"
									>Solo Work</a
								>
								<a href="/pages/music/fishercats.html" class="small-caps"
									>The Fisher Cats</a
								>
								<a href="/pages/music/thunk.html" class="small-caps">Thunk</a>
							</div>
						</li>
						<li>
							<a class="swipeOver-link" href="/pages/contact">Contact</a>
						</li>
					</ul>
				</div>
				<a class="close">&times;</a>
			</div>

"#,
        render_title(&markdown.title)
    )
}

pub fn render_title(title: &str) -> String {
    format!(
        r#"<!-- Primary Meta Tags -->
		<title>{} | Grace Bobber</title>
		<meta name="title" content="{} | Grace Bobber" />
    "#,
        title, title
    )
}

pub fn render_body(markdown: &Markdown) -> String {
    format!(
        r#"  <main class="content">
            <h2>{}</h2>
            <h5 id={}><em>{}</em></h5>
            <img src="{}" alt="{}" id="{}-in-article">
            {}
		</main>
        "#,
        markdown.title,
        markdown.date.to_string(),
        markdown.date.format("%B %e, %Y"),
        markdown.image,
        markdown.image,
        Path::new(&markdown.image)
            .file_stem()
            .unwrap()
            .to_string_lossy(),
        markdown.html_content
    )
}

pub fn render_bottom_navigation(
    next_article: Option<&Markdown>,
    previous_article: Option<&Markdown>,
    content_dir: &str,
) -> String {
    fn generate_nav_wrapper(article_string: String) -> String {
        format!(
            r#" <footer class="articleNavButtons">
            {}
			</footer>
		</section> "#,
            article_string
        )
    }
    fn generate_next_article(next_article: Option<&Markdown>, content_dir: &str) -> String {
        match next_article {
            Some(next_article) => {
                format!(
                    r#" <a href=".{}" class="nextArticle">
					<svg
						class="articleNavButtons-arrow"
						role="img"
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
						aria-labelledby="chevronLeftIconTitle"
						stroke="hsl(0, 0%, 13%)"
						stroke-width="0.6666666666666666"
						stroke-linecap="square"
						stroke-linejoin="miter"
						fill="none"
						color="hsl(0, 0%, 13%)"
					>
						<title id="chevronLeftIconTitle">Chevron Left</title>
						<polyline points="14 18 8 12 14 6 14 6" />
					</svg>
					<section class="nextArticle-text">
						<span class="small-caps">Next</span>
						<h4>{}</h4>
						<span class="small-caps">{}</span>
					</section>
				</a>
			"#,
                    next_article
                        .file_name
                        .trim_start_matches(content_dir)
                        .replace(".md", ".html"),
                    next_article.title,
                    next_article.date.format("%B %e, %Y")
                )
            }
            None => r#"
                <a href="" class="nextArticle">
					<!-- intentionally left blank -->
				</a> "#
                .to_string(),
        }
    }
    fn generate_previous_article(previous_article: Option<&Markdown>, content_dir: &str) -> String {
        match previous_article {
            Some(previous_article) => {
                format!(
                    r#" <a href=".{}" class="previousArticle">
					<section class="previousArticle-text">
						<span class="small-caps">Previous</span>
						<h4>{}</h4>
						<span class="small-caps">{}</span>
					</section>
					<svg
						class="articleNavButtons-arrow"
						role="img"
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 24 24"
						aria-labelledby="chevronRightIconTitle"
						stroke="hsl(0, 0%, 13%)"
						stroke-width="0.6666666666666666"
						stroke-linecap="square"
						stroke-linejoin="miter"
						fill="none"
						color="hsl(0, 0%, 13%)"
					>
						<title id="chevronRightIconTitle">Chevron Right</title>
						<polyline points="10 6 16 12 10 18 10 18" />
					</svg>
				</a>
		"#,
                    previous_article
                        .file_name
                        .trim_start_matches(content_dir)
                        .replace(".md", ".html"),
                    previous_article.title,
                    previous_article.date.format("%B %e, %Y")
                )
            }
            None => r#"
                <a href="" class="previousArticle">
					<!-- intentionally left blank -->
				</a> "#
                .to_string(),
        }
    }

    generate_nav_wrapper(format!(
        "{}\n{}",
        generate_next_article(next_article, content_dir),
        generate_previous_article(previous_article, content_dir)
    ))
}

pub const FOOTER: &str = r#"
		<footer class="footer">
			<div class="footer-linkList">
				<a href="https://www.facebook.com/gracebobber/" class="footer-links"
					>facebook</a
				>
				<span>|</span>
				<a href="https://www.instagram.com/gracecbobber/" class="footer-links"
					>instagram</a
				>
				<span>|</span>
				<a
					href="https://www.linkedin.com/in/grace-bobber-18478a190/"
					class="footer-links"
					>linkedin</a
				>
				<span>|</span>
				<a href="https://twitter.com/gracecbobber/" class="footer-links"
					>twitter</a
				>
				<span>|</span>
				<a
					href="https://youtube.com/channel/UCrHRRBWgNyX_rMAJCMHqaUQ"
					class="footer-links"
					>youtube</a
				>
			</div>
			<p class="footer-byline">
				Made by
				<a href="https://www.ajax.computer">Alex Jackson</a>
			</p>
			<a href="https://github.com/ajaxbits"
				><svg
					class="footer-github"
					role="img"
					viewBox="0 0 24 24"
					xmlns="http://www.w3.org/2000/svg"
				>
					<title>GitHub icon</title>
					<path
						d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"
					/></svg
			></a>
		</footer>
		<script src="../dist/scripts/menu.js"></script>
	</body>
</html>
"#;
