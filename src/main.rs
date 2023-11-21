use core::fmt::Write;
use std::{
    env,
    io::{self},
    time::SystemTime
};

use atom_syndication::{
    Content, Entry, FeedBuilder, GeneratorBuilder, LinkBuilder, TextBuilder, TextType, WriteConfig,
};
use chrono::{DateTime, Local};

mod types;
use types::ImmobiliareResponse;

// Manifest environment variables
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const NAME: &str = env!("CARGO_PKG_NAME");

// Immobiliare.it constants
const SCAM_AGENCIES: &[&str] = &[
    "Affitto Privato Parma",
    "Agenzia Informazione Casa di Dott.ssa Savi Daniela",
    "Trova Affitto Parma",
];

fn help() {
    eprintln!("Usage: immobiliare2atom [URL]\nGenerates an atom feed for the specified URL.");
    std::process::exit(2);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Check arguments number
    if args.len() < 2 {
        help();
    }

    let url = args.get(1).unwrap();
    let response = reqwest::blocking::get(url)?.json::<ImmobiliareResponse>()?;

    // Get generator
    let generator = GeneratorBuilder::default()
        .uri(Some(REPOSITORY.to_owned()))
        .version(Some(VERSION.to_owned()))
        .value(NAME.to_owned())
        .build();

    // Get links
    let feed_link = LinkBuilder::default()
        .rel("alternate".to_owned())
        .mime_type(Some("text/html".to_owned()))
        .href(url.to_owned())
        .build();

    // Get title
    let feed_title = TextBuilder::default()
        .r#type(TextType::Text)
        .value(response.seo_data.title)
        .build();

    // Get local DateTime
    let update_time: DateTime<Local> = SystemTime::now().into();

    // Build feed (except entries)
    let mut feed = FeedBuilder::default()
        .generator(Some(generator))
        .links(vec![feed_link])
        .title(feed_title)
        .updated(update_time)
        .build();

    // Store the entries array
    let mut entries: Vec<Entry> = Vec::with_capacity(response.results.len());

    // Parse feed items
    for result in response.results {
        let mut entry = Entry::default();
        let mut content = Content::default();
        content.set_content_type(Some("xhtml".to_owned()));
        let mut description = r#"<div xmlns="http://www.w3.org/1999/xhtml">"#.to_owned();

        // Get estate agency
        if let Some(agency) = result.real_estate.advertiser.agency {
            let agency = agency.display_name;
            if SCAM_AGENCIES.contains(&agency.as_str()) {
                continue;
            }

            write!(description, "<p>Agenzia: {agency}</p>")?;
        }

        // Get title
        entry.set_title(result.real_estate.title);

        // Get link/id
        let item_url = result.seo.url;

        let link = LinkBuilder::default()
            .rel("alternate".to_owned())
            .mime_type(Some("text/html".to_owned()))
            .href(item_url.clone())
            .build();

        entry.set_links([link]);
        entry.set_id(item_url);

        // Get price
        let price = result.real_estate.price.formatted_value;
        write!(description, "<p>Prezzo: {price}</p>")?;

        // Get properties
        if let Some(properties) = result.real_estate.properties.first() {
            // Get number of rooms
            let rooms = &properties.rooms;
            write!(description, "<p>Locali: {rooms}</p>")?;

            // Get surface
            let surface = &properties.surface;
            write!(description, "<p>Superficie: {surface}</p>")?;

            // Get bathrooms
            if let Some(bathrooms) = properties
                .bathrooms
                .as_ref()
                .or(properties.ga4_bathrooms.as_ref())
            {
                write!(description, "<p>Bagni: {bathrooms}</p>")?;
            }

            // Get floor
            if let Some(floor) = &properties.floor {
                let floor = &floor.value;
                write!(description, "<p>Piano: {floor}</p>")?;
            }

            // Get description
            if let Some(desc) = &properties.description {
                write!(description, "<p>{desc}</p>")?;
            }
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
