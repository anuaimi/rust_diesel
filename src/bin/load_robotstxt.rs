// use texting_robots::get_robots_url;
// use texting_robots::{Robot, get_robots_url};
use reqwest;
use std::error::Error;
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

fn main() -> Result<(), Box<dyn Error>> {

  let url = "https://lonelyplanet.com";
  let robots_url = get_robots_url(url).unwrap();
  // println!("robotstxt: : {}", robots_url);

  // get the file
  // CHECK FOR 404??
  let body = reqwest::blocking::get(robots_url)?.text()?;

  // println!("body = {}", body);

  // parse the robots.txt
  let r = Robot::new("travel", body.as_bytes()).unwrap();

  // get the list of sitemaps
  println!("{}", r.sitemaps.len());
  // println!("{:?}", r.sitemaps);

  for sitemap in r.sitemaps {
    println!("loading {}", sitemap);

    // load sitemap
    let file_text = reqwest::blocking::get(sitemap)?.text()?;
    
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