use rustful::{Context, Response};
use rustful::header::ContentType;
use rustful::StatusCode;

use std::fs::File;
use std::io::Read;

macro_rules! render_file {
    ($page:expr, $response:expr) => {{
        let mut file = File::open($page).unwrap();
        let mut html = String::new();

        file.read_to_string(&mut html).unwrap();
        $response.send(format!("{}", html))
    }};
}

macro_rules! set_content_type {
    ($response:expr, $mime_type:ident / $subtype:ident) => {
        $response.headers_mut().set(ContentType(content_type!($mime_type / $subtype; Charset = Utf8)))
    };
}

pub fn averages(_context: Context, mut response: Response) {
    set_content_type!(response, Text / Html);
    render_file!("views/averages.html", response)
}

pub fn batters(_context: Context, mut response: Response) {
    set_content_type!(response, Text / Html);
    render_file!("views/batters.html", response);
}

pub fn player(_context: Context, mut response: Response) {
    set_content_type!(response, Text / Html);
    render_file!("views/player.html", response)
}

pub fn league(_context: Context, mut response: Response) {
    set_content_type!(response, Text / Html);
    render_file!("views/league.html", response)
}

pub fn tags(_context: Context, mut response: Response) {
    set_content_type!(response, Text / Html);
    render_file!("views/tags.html", response)
}

pub fn team(_context: Context, mut response: Response) {
    set_content_type!(response, Text / Html);
    render_file!("views/team.html", response)
}

pub fn static_file(context: Context, mut response: Response) {
    match context.variables.get("file") {
        Some(file) => {
            set_content_type!(response, Text / Css);
            render_file!(format!("static/{}", file), response)
        },
        None => {
            set_content_type!(response, Text / Plain);
            response.send("File not found")
        }
    }
}

pub fn error(_context: Context, mut response: Response) {
    response.set_status(StatusCode::NotFound);
    set_content_type!(response, Text / Html);
    response.send("<img src=\"http://cf.chucklesnetwork.com/items/3/8/5/8/3/original/i-dont-always-crash-error-404-but-when-i-do-not-found.jpg\">")
}
