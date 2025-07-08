use gloo_net::http::Request;
use leptos::{prelude::*, task::spawn_local};
#[cfg(feature = "ssr")]
use pulldown_cmark::{html::push_html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd, CowStr};

#[cfg(feature = "ssr")]
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

// fn markdown_to_html(md: &str) -> String {
//     let mut options = Options::empty();
//     options.insert(Options::ENABLE_STRIKETHROUGH);
//     options.insert(Options::ENABLE_TABLES);
//     options.insert(Options::ENABLE_FOOTNOTES);
//     options.insert(Options::ENABLE_TASKLISTS);

//     let parser = Parser::new_ext(md, options);
//     let mut html_output = String::new();
//     push_html(&mut html_output, parser);
//     html_output
// }

#[server]
pub async fn markdown_to_html(md: String) -> Result<String, ServerFnError> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&md, options);

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    let mut html_output = String::new();
    let mut buffer = String::new();
    let mut in_code_block = false;
    let mut lang = String::new();
    let mut events = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                in_code_block = true;
                lang = info.to_string();
                buffer.clear();
            }
            Event::Text(text) if in_code_block => {
                buffer.push_str(&text);
            }
            Event::End(TagEnd::CodeBlock) if in_code_block => {
                in_code_block = false;
                let syntax = ps.find_syntax_by_token(&lang).unwrap_or_else(|| ps.find_syntax_plain_text());
                let highlighted = syntect::html::highlighted_html_for_string(&buffer, &ps, syntax, theme).unwrap();
                events.push(Event::Html(CowStr::Boxed(highlighted.into_boxed_str())));
            }
            _ if !in_code_block => {
                events.push(event);
            }
            _ => {}
        }
    }

    push_html(&mut html_output, events.into_iter());
    Ok(html_output)
}

#[allow(non_snake_case)]
#[component]
pub fn MarkdownFromUrl(url: RwSignal<Option<String>>) -> impl IntoView {
    let (content, set_content) = signal::<Option<String>>(None);
    let (html, set_html) = signal::<Option<String>>(None);
    let (error, set_error) = signal::<Option<String>>(None);

    spawn_local({
        let set_content = set_content.clone();
        let set_error = set_error.clone();
        async move {
            if url.get().is_none() {
                set_error.set(Some("URL is empty".to_string()));
            } else {
                match Request::get(url.get().unwrap().as_str()).send().await {
                    Ok(res) => match res.text().await {
                        Ok(text) => set_content.set(Some(text)),
                        Err(e) => set_error.set(Some(format!("Error reading text: {e}"))),
                    },
                    Err(e) => set_error.set(Some(format!("Fetch error: {e}"))),
                }
            }
        }
    });

    // Convert markdown ke HTML pake server function
    Effect::new(move |_| {
        if let Some(md) = content.get() {
            let set_html = set_html.clone();
            let set_error = set_error.clone();
            spawn_local(async move {
                match markdown_to_html(md).await {
                    Ok(html_result) => set_html.set(Some(html_result)),
                    Err(e) => set_error.set(Some(format!("Markdown parsing error: {e}"))),
                }
            });
        }
    });

    view! {
        <div class="markdown-content prose max-w-none">
            <Show
                when=move || html.get().is_some()
                fallback=move || view! { <p>Loading markdown...</p> }
            >
                {move || view! { <div inner_html=html.get().unwrap()></div> }}
            </Show>

            {move || error.get().map(|e| view! { <p class="text-danger">Error: {e}</p> })}
        </div>
    }
}