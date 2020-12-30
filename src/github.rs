use std::collections::HashMap;
use prettytable::format::consts::FORMAT_BOX_CHARS;
use prettytable::Table;
use colored::*;


pub async fn get_langs(handle: String) {

  let client = reqwest::Client::new();
  let url = format!("https://api.github.com/repos/{}/languages", handle);

  let langs: HashMap<String, usize> = client.get(&url)
    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.95 Safari/537.36")
    .send().await.unwrap()
    .json()
    .await
    .unwrap();

  println!("{}\n", handle.bold().underline());

  let mut langs: Table = langs.into_iter().map(|(k, v)| row![bFg->k, Fy->v] ).collect();
  langs.set_format(*FORMAT_BOX_CHARS);
  langs.printstd();
}