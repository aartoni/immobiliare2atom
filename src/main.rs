use std::{
    io::{self, Read},
    time::SystemTime,
};

use atom_syndication::{
    Content, Entry, FeedBuilder, GeneratorBuilder, LinkBuilder, TextBuilder, TextType, WriteConfig,
};
use chrono::{DateTime, Local};
use regex::Regex;
use scraper::{Html, Selector};

// Manifest environment variables
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const NAME: &str = env!("CARGO_PKG_NAME");

// Immobiliare.it constants
const SEARCH_RESULTS: usize = 25;
const FEED_TITLE_QUERY: &str = "title";
const ITEMS_QUERY: &str = ".in-realEstateListCard";
const TITLE_QUERY: &str = ".in-card__title";
const PRICE_QUERY: &str = ".in-realEstateListCard__priceOnTop";
const ROOMS_QUERY: &str = "li[aria-label=locali]";
const SURFACE_QUERY: &str = "li[aria-label=superficie]";
const BATHROOMS_QUERY: &str = "li[aria-label=bagno]";
const FLOOR_QUERY: &str = "li[aria-label=piano]";
const AGENCY_QUERY: &str = ".nd-figure__content";
const DESCRIPTION_QUERY: &str = ".in-realEstateListCard__description";
const SCAM_AGENCIES: &'static [&str] = &[
    "Affitto Privato Parma",
    "Agenzia Informazione Casa di Dott.ssa Savi Daniela",
    "Trova Affitto Parma",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get document
    let mut html = String::new();
    io::stdin().read_to_string(&mut html)?;
    let document = Html::parse_document(&html);

    // Get feed data
    let feed_title_selector = Selector::parse(FEED_TITLE_QUERY)?;
    let feed_title_input = document.select(&feed_title_selector).next().unwrap();
    let feed_title = feed_title_input.text().next().unwrap();

    // Get feed link
    let link_regex = Regex::new("(https://.*)&amp;mode=rss")?;
    let feed_link = link_regex.captures(&html).unwrap().get(1).unwrap().as_str();

    // Get generator
    let generator = GeneratorBuilder::default()
        .uri(Some(REPOSITORY.to_string()))
        .version(Some(VERSION.to_string()))
        .value(NAME.to_string())
        .build();

    // Get links
    let link = LinkBuilder::default()
        .rel("alternate".to_string())
        .mime_type(Some("text/html".to_string()))
        .href(feed_link.to_string())
        .build();

    // Get title
    let title = TextBuilder::default()
        .r#type(TextType::Text)
        .value(feed_title.to_string())
        .build();

    // Get local DateTime
    let update_time: DateTime<Local> = SystemTime::now().into();

    // Build feed (except entries)
    let mut feed = FeedBuilder::default()
        .generator(Some(generator))
        .links(vec![link])
        .title(title)
        .updated(update_time)
        .build();

    // Get item selectors and regexes
    let agency_selector = Selector::parse(AGENCY_QUERY)?;
    let title_selector = Selector::parse(TITLE_QUERY)?;
    let price_selector = Selector::parse(PRICE_QUERY)?;
    let rooms_selector = Selector::parse(ROOMS_QUERY)?;
    let surface_selector = Selector::parse(SURFACE_QUERY)?;
    let bathrooms_selector = Selector::parse(BATHROOMS_QUERY)?;
    let floor_selector = Selector::parse(FLOOR_QUERY)?;
    let description_selector = Selector::parse(DESCRIPTION_QUERY)?;
    let items_selector = Selector::parse(ITEMS_QUERY)?;

    // Store the entries array
    let mut entries: Vec<Entry> = Vec::with_capacity(SEARCH_RESULTS);

    // Parse feed items
    for item in document.select(&items_selector) {
        let mut entry = Entry::default();
        let mut content = Content::default();
        content.set_content_type(Some("xhtml".to_string()));
        let mut description = r#"<div xmlns="http://www.w3.org/1999/xhtml">"#.to_string();

        // Get estate agency
        if let Some(agency) = item.select(&agency_selector).next() {
            let agency = agency.value().attr("alt").unwrap();

            if SCAM_AGENCIES.contains(&agency) {
                continue;
            } else if agency != "" {
                description.push_str(&format!("<p>Agenzia: {agency}</p>"));
            }
        }

        // Get title
        let title_element = item.select(&title_selector).next().unwrap();
        let title = title_element.text().last().unwrap();
        entry.set_title(title);

        // Get link/id
        let item_url = title_element.value().attr("href").unwrap();

        let link = LinkBuilder::default()
            .rel("alternate".to_string())
            .mime_type(Some("text/html".to_string()))
            .href(item_url.to_string())
            .build();

        entry.set_links([link]);
        entry.set_id(item_url);

        // Get price
        let price = item
            .select(&price_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();

        description.push_str(&format!("<p>Prezzo: {price}</p>"));

        // Get number of rooms
        if let Some(rooms) = item.select(&rooms_selector).next() {
            let rooms = rooms.text().next().unwrap();
            description.push_str(&format!("<p>Locali: {rooms}</p>"));
        }

        // Get surface
        if let Some(surface) = item.select(&surface_selector).next() {
            let surface = surface.text().next().unwrap();
            description.push_str(&format!("<p>Superficie: {surface}</p>"));
        }

        // Get bathrooms
        if let Some(bathrooms) = item.select(&bathrooms_selector).next() {
            let bathrooms = bathrooms.text().next().unwrap();
            description.push_str(&format!("<p>Bagni: {bathrooms}</p>"));
        }

        // Get floor
        if let Some(floor) = item.select(&floor_selector).next() {
            let floor = floor.text().next().unwrap();
            description.push_str(&format!("<p>Piano: {floor}</p>"));
        }

        // Get description
        if let Some(desc) = item.select(&description_selector).next() {
            let desc = desc.text().next().unwrap();
            description.push_str(&format!("<p>{desc}</p>"));
        }

        // Finish and append entry
        description.push_str("</div>");
        content.set_value(description);
        entry.set_content(content);
        entry.set_updated(update_time);
        entries.push(entry);
    }

    feed.set_entries(entries);

    let write_config = WriteConfig {
        write_document_declaration: true,
        indent_size: Some(2),
    };

    feed.write_with_config(io::stdout(), write_config)?;
    Ok(())
}
