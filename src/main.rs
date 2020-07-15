use hyper::{Body, Request, Response, Server, Method, StatusCode, header, HeaderMap};
use hyper::service::{make_service_fn, service_fn};
use serde::{Serialize, Deserialize};
use bytes::buf::BufExt;
use scraper::{Html, Selector, ElementRef};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

#[derive(Serialize, Deserialize, Debug)]
struct IncomingBody {
    pub dom: String
}

#[derive(Serialize, Deserialize, Debug)]
struct GameDetails {
    url: String,
    id: String,
    name: String,
    r#type: String
}

fn format_game_detail(href: &str, anchor: &ElementRef) -> GameDetails {
    let ending_re = Regex::new(r"/home$").unwrap();
    let id_re = Regex::new(r"/(product|bundles)/(.*)/?").unwrap();
    let clean_url = ending_re.replace(href, "");
    let clean_string = (&clean_url).to_string();
    let id_cap = id_re.captures(&clean_string).unwrap();

    let title_selector = Selector::parse("div > div > div > span:first-child").unwrap();
    let title = anchor.select(&title_selector).next().unwrap();

    GameDetails {
        url: String::from(href),
        id: (&id_cap[2]).to_string(),
        name: title.inner_html(),
        r#type: (&id_cap[1]).to_string()
    }
}

fn create_game_details(dom: &String) -> Vec<GameDetails> {
    let fragment = Html::parse_fragment(dom);
    let selector = Selector::parse("main > div > div > div:last-child section > ul > li > a").unwrap();
    let mut game_details: Vec<GameDetails> = Vec::new();

    for element in fragment.select(&selector) {
        let href: &str = element.value().attr("href").unwrap();
        game_details.push(format_game_detail(&href, &element));
    }

    game_details
}

async fn process_dom(req: Request<Body>) -> Result<Response<Body>> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::POST, "/scrape") => {
            let path = Path::new("scrapped-text.json");
            let display = path.display();

            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why),
                Ok(file) => file,
            };

            let whole_body = hyper::body::aggregate(req).await?;
            let data: IncomingBody = serde_json::from_reader(whole_body.reader())?;
            let game_details = create_game_details(&data.dom);
            let details_json = serde_json::to_string(&game_details)?;

            file.write(details_json.as_bytes())?;

            let json = serde_json::to_string(&data)?;
            *response.body_mut() = Body::from(json);
            let mut map = HeaderMap::new();
            map.insert("Content-Type", header::HeaderValue::from_static("application/json"));
            map.insert("Access-Control-Allow-Origin", header::HeaderValue::from_static("*"));
            *response.headers_mut() = map;
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| async {
        Ok::<_, GenericError>(service_fn(process_dom))
    });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
