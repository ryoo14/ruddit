use roux::Subreddit;
use tokio;

#[tokio::main]
async fn main() {
    let subreddit = Subreddit::new("battlestations");
    let top = subreddit.top(10, None).await.unwrap();
    for i in top.data.children {
        if let Some(v) = i.data.url {
            println!("{} : {}", i.data.title, v);
        }
    }
}
