mod trello;
mod config;
mod notify;

use trello::Trello;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Opt {
    Trello {
        url: String,
    }
}

#[tokio::main]
async fn main() {
    let opts = Opt::from_args();
    let config = config::parse();

    match opts {
        Opt::Trello { url } => {

            let trello = Trello::new(&config.trello);

            trello.bookmark(url).await;
        }
    }
}
