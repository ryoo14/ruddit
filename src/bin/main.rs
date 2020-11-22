use roux::{Me, Reddit, Subreddit, User};
use ruddit::{setting::Setting, subscribe_data::RedditResponse};
use tokio;
use ureq;

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

    // load setting for reddit api
    let setting = Setting::load_toml("Setting.toml");

    // create Reddit instatnce
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

    // request to get user subscribed subreddit
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
