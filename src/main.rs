extern crate clap;
extern crate reqwest;
extern crate scraper;

use clap::{App, Arg, SubCommand};
use scraper::{Html, Selector};
use std::collections::HashMap;

fn main() {
	// naive cache
	let cache: HashMap<String, Vec<String>> = HashMap::new();

	let matches = App::new("wiki-crawler")
		.author("Ryan Faulhaber")
		.arg(
			Arg::with_name("start_url")
				.required(true)
				.help("the starting Wikipedia page")
				.validator(valid_url),
		)
		.arg(
			Arg::with_name("end_url")
				.required(true)
				.help("the ending Wikipedia page")
				.validator(valid_url),
		)
		.get_matches();

	let start_url = matches.value_of("start_url").unwrap();
	let end_url = matches.value_of("end_url").unwrap();

	println!("starting with: {}", start_url);
	println!("ending with: {}", end_url);

	let resp_result = reqwest::get(start_url);

	let page = match resp_result {
		Ok(mut resp) => resp.text().unwrap(),
		Err(err) => panic!(err),
	};

	let html_doc = Html::parse_document(page.as_str());
	let a_tags = Selector::parse("a").unwrap();

	let valid_links = html_doc
		.select(&a_tags)
		.map(|element| element.value().attr("href"))
		.filter(|href| href.is_some())
		.map(|href_op| href_op.unwrap())
		.filter(|href_str| {
			// this is quite ugly
			href_str.starts_with("/wiki/")
				&& !href_str.contains("Wikipedia")
				&& !href_str.contains("Category")
				&& !href_str.contains("Portal")
				&& !href_str.contains("Special")
				&& !href_str.contains("File:")
				&& !href_str.contains("Talk:")
				&& !href_str.contains("Help:")
		});

	for link in valid_links {
		println!("link: {}", link);
	}
}

fn valid_url(v: String) -> Result<(), String> {
	if v.contains("https://en.wikipedia.org/wiki/") {
		return Ok(());
	}

	Err(String::from("Not a valid Wikipedia URL"))
}
