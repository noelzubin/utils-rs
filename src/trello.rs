use serde::{Deserialize};
use crate::config::Trello as TrelloConfig;
use crate::notify;

pub struct Trello<'a> {
    pub config: &'a TrelloConfig,
}

impl<'a> Trello<'a> {
    pub fn new(config: &'a TrelloConfig ) -> Self {
        Trello { config }
    }

    async fn add_url(&self, url: &str) -> reqwest::Result<()> {
        let post_url = format!("https://api.trello.com/1/cards?key={}&token={}&idList={}&urlSource={}", self.config.key, self.config.token, self.config.list_id, url);
        let client = reqwest::Client::new();
        client.post(&post_url).send().await?;
        Ok(())
    }

    pub async fn bookmark(&self, url: String) {
        notify::show("Saving Bookmark", &url);

        let search_url = format!("https://api.trello.com/1/search?token={}&key={}&idBoards={}&query={}&modelTypes=cards", self.config.token,  self.config.key, self.config.board_id, &url);
        let resp = reqwest::get(&search_url).await.unwrap().json::<Resp>().await.unwrap();
        if resp.cards.len() > 0 {
            notify::show("Saving Bookmark", "bookmark alread exists.");
        } else {
            self.add_url(&url).await.unwrap();
            notify::show("Saving Bookmark", "saved bookmark.");
        }
    }
}


#[derive(Deserialize, Debug)]
struct Resp {
    cards: Vec<CardResp>
}

#[derive(Deserialize, Debug)]
struct CardResp {
    id: String,
}