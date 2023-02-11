use std::{io::{BufReader, Read}, fs::File};

use crate::util::Sink;


pub fn local_fetch_quake_data(path: &'static str) -> Result<String, ()> {
    let mut buf = String::new();
    let mut reader = BufReader::new(File::open(path).unwrap());
    reader.read_to_string(&mut buf).unwrap();
    Ok(buf)
}

pub fn remote_fetch_quake_data(url: &'static str) -> Result<String, ()> {
    fn get_content(url: &str) -> Result<String, ()> {
        let response = reqwest::blocking::get(url).unwrap();
        Ok(response.text().unwrap())
    }

    fn get_quake_text(html: &str) -> String {
        html.lines()
            .skip_while(|line| line.trim() != "<pre>")
            .skip(1)
            .take_while(|line| line.trim() != "</pre>")
            .fold("".to_string(), |mut acc, line| {
                acc.push_str(line);
                acc.push('\n');
                acc
            })
    }

    let html = get_content(url);
    html.map(|h| get_quake_text(&h))
}

pub enum Source {
    Local(&'static str),
    Remote(&'static str), // URL
}

pub fn fetch_quake_data(source: Source) -> Result<String, ()> {
    match source {
        Source::Local(path) => local_fetch_quake_data(path),
        Source::Remote(url) => remote_fetch_quake_data(url),
    }
}