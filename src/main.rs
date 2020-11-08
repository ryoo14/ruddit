//use config::Config;
//use roux::Reddit;
use roux::Subreddit;
use tokio;

#[tokio::main]
async fn main() {
    let subreddit = Subreddit::new("battlestations");
    let top = subreddit.top(25, None).await;
    println!("{:?}", top.unwrap());
}
