use roux::{Me, Reddit, Subreddit, User};
use serde::Deserialize;
use std::fs;
use tokio;
use toml;
use ureq;

// subscriber
type RedditResponse = BasicListing<RedditResponseData>;
type BasicListing<T> = BasicThing<Listing<BasicThing<T>>>;

#[derive(Deserialize)]
struct RedditResponseData {
    display_name: String,
}

#[derive(Deserialize)]
struct BasicThing<T> {
    kind: String,
    data: T,
}

#[derive(Deserialize)]
struct Listing<T> {
    modhash: Option<String>,
    dist: Option<i32>,
    after: Option<String>,
    before: Option<String>,
    children: Vec<T>,
}

// settings
#[derive(Deserialize)]
struct Setting {
    user_agent: String,
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    // // display subreddit contents
    // let subreddit = Subreddit::new("battlestations");
    // let top = subreddit.top(10, None).await.unwrap();
    // for i in top.data.children {
    //     if let Some(v) = i.data.url {
    //         println!("{} : {}", i.data.title, v);
    //     }
    // }

    // // display user info
    // let user = User::new("ryoana");
    // let view = user.overview().await.unwrap();
    // for i in view.data.children {
    //     println!("{:?}", i.data.subreddit);
    // }

    let setting_str: String = fs::read_to_string("Setting.toml").unwrap();
    let setting: Setting = toml::from_str(&setting_str).unwrap();

    let client = Reddit::new(
        &setting.user_agent,
        &setting.client_id,
        &setting.client_secret,
    )
    .username(&setting.username)
    .password(&setting.password)
    .login()
    .await;

    let me = client.unwrap();

    let access_token = "bearer ".to_string() + &me.access_token;
    let url = String::from("https://oauth.reddit.com/subreddits/mine/subscriber");
    let resp = ureq::get(&url).set("Authorization", &access_token).call();

    if resp.ok() {
        let json = resp.into_json_deserialize::<RedditResponse>().unwrap();
        for i in json.data.children {
            println!("{}", i.data.display_name);
        }
    } else {
        println!("{}", String::from("NG!"));
    }
}
