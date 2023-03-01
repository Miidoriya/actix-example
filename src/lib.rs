#[macro_use]
extern crate lazy_static;

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, sync::Arc};

pub const CORE_URL: &str = "https://comicbookroundup.com";

const MARVEL_URL: &str =
    "https://comicbookroundup.com/comic-books/reviews/marvel-comics/all-series"; //
const DC_URL: &str = "https://comicbookroundup.com/comic-books/reviews/dc-comics/all-series"; //
const IMAGE_URL: &str = "https://comicbookroundup.com/comic-books/reviews/image-comics/all-series"; //
const IDW_URL: &str = "https://comicbookroundup.com/comic-books/reviews/idw-publishing/all-series"; //
const DARK_HORSE_URL: &str =
    "https://comicbookroundup.com/comic-books/reviews/dark-horse-comics/all-series"; //
const BOOM_URL: &str = "https://comicbookroundup.com/comic-books/reviews/boom-studios/all-series"; //
const DYNAMITE_URL: &str =
    "https://comicbookroundup.com/comic-books/reviews/dynamite-entertainment/all-series"; //
const VALIANT_URL: &str =
    "https://comicbookroundup.com/comic-books/reviews/valiant-comics/all-series"; //
const VERTIGO_URL: &str = "https://comicbookroundup.com/comic-books/reviews/vertigo/all-series"; //
const ONI_URL: &str = "https://comicbookroundup.com/comic-books/reviews/oni-press/all-series"; //
const AFTERSHOCK_URL: &str =
    "https://comicbookroundup.com/comic-books/reviews/aftershock-comics/all-series"; //
const ARCHIE_URL: &str =
    "https://comicbookroundup.com/comic-books/reviews/archie-comics/all-series"; //
const TITAN_URL: &str = "https://comicbookroundup.com/comic-books/reviews/titan-books/all-series"; //
const ZENESCOPE_URL: &str =
    "https://comicbookroundup.com/comic-books/reviews/zenescope-entertainment/all-series"; //
const BLACK_MASK_URL: &str =
    "https://comicbookroundup.com/comic-books/reviews/black-mask-studios/all-series"; //
const RED_5_URL: &str = "https://comicbookroundup.com/comic-books/reviews/red-5-comics/all-series"; //
const VAULT_URL: &str = "https://comicbookroundup.com/comic-books/reviews/vault-comics/all-series"; //

// Create a map of publisher names to their respective urls
lazy_static! {
    static ref PUBLISHER_URLS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("marvel", MARVEL_URL);
        m.insert("dc", DC_URL);
        m.insert("image", IMAGE_URL);
        m.insert("idw", IDW_URL);
        m.insert("dark horse", DARK_HORSE_URL);
        m.insert("boom", BOOM_URL);
        m.insert("dynamite", DYNAMITE_URL);
        m.insert("valiant", VALIANT_URL);
        m.insert("vertigo", VERTIGO_URL);
        m.insert("oni", ONI_URL);
        m.insert("aftershock", AFTERSHOCK_URL);
        m.insert("archie", ARCHIE_URL);
        m.insert("titan", TITAN_URL);
        m.insert("zenescope", ZENESCOPE_URL);
        m.insert("black mask", BLACK_MASK_URL);
        m.insert("red 5", RED_5_URL);
        m.insert("vault", VAULT_URL);
        m
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComicInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    writers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artists: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cover_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    critic_review_count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_review_count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    critic_review_score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_review_score: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ComicInfos {
    #[serde(skip_serializing_if = "Option::is_none")]
    comic_infos: Option<Vec<ComicInfo>>,
}

#[derive(Debug)]
struct IdName {
    id: Option<String>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComicsRequestBody {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comic {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublisherComics {
    pub comics: Vec<Comic>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComicIssues {
    pub urls: Vec<String>,
}

pub async fn get_comic_issue_json_response(url: &str) -> Result<ComicInfo, Box<dyn Error>> {
    let response = get_response(url).await?;
    let id_name = parse_name(&response).unwrap();
    let id = id_name.id;
    let name = id_name.name;
    let writers: Option<Vec<String>> =
        parse_comic_info_field(&response, "Writer")
            .unwrap()
            .map(|writers| {
                writers
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            });
    let artists = parse_comic_info_field(&response, "Artist")
        .unwrap()
        .map(|artists| {
            artists
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        });
    let publisher = parse_comic_info_field(&response, "Publisher").unwrap();
    let release_date = parse_comic_info_field(&response, "Release Date").unwrap();
    let cover_price = parse_comic_info_field(&response, "Cover Price").unwrap();
    let critic_review_count = parse_review_count(&response, "Critic Reviews").unwrap();
    let user_review_count = parse_review_count(&response, "User Reviews").unwrap();
    let critic_review_score = parse_review_score(&response, "Critic Rating").unwrap();
    let user_review_score = parse_review_score(&response, "User Rating").unwrap();
    let comic_info = ComicInfo {
        id,
        name,
        writers,
        artists,
        publisher,
        release_date,
        cover_price,
        critic_review_count,
        user_review_count,
        critic_review_score,
        user_review_score,
    };
    Ok(comic_info)
}

async fn get_response(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

fn parse_name(response: &str) -> Result<IdName, scraper::error::SelectorErrorKind> {
    let response = scraper::Html::parse_document(response);
    let id_selector = scraper::Selector::parse("div.series-buttons a.series")?;
    let name_selector = scraper::Selector::parse(".issue div.container div.right h1 span")?;
    let name = response
        .select(&name_selector)
        .next()
        .map(|name| name.text().collect::<Vec<_>>().join(""));
    let id = response.select(&id_selector).next().map(|name| {
        name.value()
            .attr("href")
            .unwrap()
            .to_string()
            .split('/')
            .last()
            .unwrap()
            .to_string()
    });
    let id_name = IdName { id, name };
    Ok(id_name)
}

fn parse_comic_info_field<'a>(
    response: &'a str,
    field: &'a str,
) -> Result<Option<String>, scraper::error::SelectorErrorKind<'a>> {
    let response = scraper::Html::parse_document(response);
    let name_selector = scraper::Selector::parse(".issue div.container div.right div.left span")?;
    let field = response
        .select(&name_selector)
        .find(|n| Arc::new(n.text().collect::<Vec<_>>().join("")).contains(field))
        .map(|name| name.text().collect::<Vec<_>>().last().unwrap().to_string());
    Ok(field)
}

fn parse_review_count<'a>(
    response: &'a str,
    field: &'a str,
) -> Result<Option<String>, scraper::error::SelectorErrorKind<'a>> {
    let re = Regex::new(r"(?x)(?P<count>\d+)").unwrap();
    let response = scraper::Html::parse_document(response);
    let name_selector = scraper::Selector::parse(".divider div.container ul.tabs li")?;
    let field = response
        .select(&name_selector)
        .find(|n| Arc::new(n.text().collect::<Vec<_>>().join("")).contains(field))
        .map(|name| {
            let val = name.text().collect::<Vec<_>>().last().unwrap().to_string();
            let caps = re.captures(&val).unwrap();
            caps["count"].to_string()
        });
    Ok(field)
}

fn parse_review_score<'a>(
    response: &'a str,
    field: &'a str,
) -> Result<Option<String>, scraper::error::SelectorErrorKind<'a>> {
    let re = Regex::new(r"(?x)(?P<score>\d+\.\d|\d+)").unwrap();
    let response = scraper::Html::parse_document(response);
    let name_selector = scraper::Selector::parse(".issue div.container div.right div.right div")?;
    let field = response
        .select(&name_selector)
        .find(|n| Arc::new(n.text().collect::<Vec<_>>().join("")).contains(field))
        .map(|name| {
            let val = name.text().collect::<Vec<_>>().join("");
            let caps = re.captures(&val);
            let a = match caps {
                Some(caps) => caps["score"].to_string(),
                None => "0".to_string(),
            };
            a
        });
    Ok(field)
}

// TODO: refactor
pub async fn parse_comic_issue_urls(url: &str) -> Result<ComicIssues, Box<dyn Error>> {
    println!("url: {}", url);
    let response_str = get_response(url).await?;
    let html_resp = scraper::Html::parse_document(&response_str);
    let id_selector =
        scraper::Selector::parse("div.section > table > tbody > tr > td.issue > a").unwrap();
    let urls: ComicIssues = ComicIssues {
        urls: html_resp
            .select(&id_selector)
            .map(|e| {
                format!(
                    "{}{}",
                    CORE_URL,
                    e.value().attr("href").map(str::to_owned).unwrap()
                )
            })
            .collect(),
    };
    Ok(urls)
}

pub async fn parse_comic_urls(name: &str) -> Result<PublisherComics, Box<dyn Error>> {
    let url = PUBLISHER_URLS.get(&name).unwrap();
    let response_str = reqwest::get(*url).await?.text().await?;
    let html_resp = scraper::Html::parse_document(&response_str);
    let id_selector =
        scraper::Selector::parse("div.section > table > tbody > tr > td.series > a").unwrap();
    let urls: PublisherComics = PublisherComics {
        comics: html_resp
            .select(&id_selector)
            .map(|e| {
                let href = e.value().attr("href");
                let href = href.map(str::to_owned);
                let text = e.text().collect::<Vec<_>>()[0].to_owned();
                Comic {
                    name: text,
                    url: format!("{}{}", CORE_URL, href.unwrap()),
                }
            })
            .collect(),
    };
    Ok(urls)
}
