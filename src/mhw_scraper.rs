use scraper::{Html, Selector};
use reqwest;

pub fn load_page(url: &str) -> Result<String, reqwest::Error>
{
    let mut result = reqwest::get(url)?;

    return result.text()
}

pub fn scrape_monster(html_text: &String) -> Result<(), String>
{
    let document = Html::parse_fragment(html_text);
    let header_selector = Selector::parse("h1,h4").unwrap();
    let mut found = document.select(&header_selector);
    
    while let Some(elem) = found.next() {
        println!("{:?}", elem.inner_html());
    }

    Ok(())
}