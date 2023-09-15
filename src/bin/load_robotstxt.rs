// use texting_robots::get_robots_url;
// use texting_robots::{Robot, get_robots_url};
use reqwest;
use std::error::Error;
use std::env;
use serde::Deserialize;
use texting_robots::{Robot, get_robots_url};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
struct SitemapIndex {
  sitemap: Vec<Sitemap>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Sitemap {
  loc: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Urlset {
  url: Vec<Url>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
struct Url {
  loc: String,
}

fn get_hostname() -> String {

  let args: Vec<String> = env::args().collect();
  if (args.len() == 1) || (args.len() > 2) {
    panic!{"usage: load_robotstxt hostname"};
  }
  
  let mut hostname = args[1].to_owned();
  if !hostname.contains("://") {
    hostname.insert_str( 0,"https://");
  }

  return hostname;
}

fn main() -> Result<(), Box<dyn Error>> {

  let hostname = get_hostname();

  println!("loading robotstxt for: {}", hostname);
  let robots_url = get_robots_url(&hostname).unwrap();

  // get the robots.txt file
  let response= reqwest::blocking::get(robots_url)?;
  let status = response.status();
  if !status.is_success() {
    // no robots.txt or some sort of error
    panic!("could not load robots.txt");
  }

  // parse the robots.txt
  let body = response.text()?;
  let r = Robot::new("travel", body.as_bytes()).unwrap();

  // get the list of sitemaps
  let mut sitemap_list = Vec::new();

  if r.sitemaps.len() == 0 {

    // we should just try the defaul /sitemap.xml
    let sitemap_url = hostname + "/sitemap.xml";
    let response = reqwest::blocking::get(&sitemap_url);
    if response.is_err() {
      panic!("no sitemaps listed in robots.txt");
    }
    // ok we found sitemap
    sitemap_list.push(sitemap_url);
  } else {
    sitemap_list = r.sitemaps;
  }
  // println!("{:?}", r.sitemaps);

  for sitemap in sitemap_list {
    println!("loading {}", sitemap);

    // load sitemap
    let file_text = reqwest::blocking::get(sitemap)?.text()?;
    
    // is it xml? normally yes but possible just text file
    if file_text.contains("xmlns") {

    } else {
      // might just be text
      panic!("sitemap is just text file - not currently supported");
    }

    // could be a siteindex or a sitemap
    if file_text.contains("sitemapindex") {

      // process it!!
      let sitemaps = serde_xml_rs::from_str::<SitemapIndex>(file_text.as_str()).unwrap();

      for sitemap in sitemaps.sitemap {
        println!("  {}", sitemap.loc);
      }

    } else if file_text.contains("urlset") {
      // process it!!
      let sites = serde_xml_rs::from_str::<Urlset>(file_text.as_str()).unwrap();

      for site in sites.url {
        println!("  {}", site.loc);
      }
        
    } else {
      println!("invalid file format");
    }
  }

  Ok(())

}