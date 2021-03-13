use roux::{Me, Reddit, Subreddit, User};
use ruddit::{setting::Setting, subscribe_data::RedditResponse};
use std::io;
use termion::raw::IntoRawMode;
use tokio;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;
use ureq;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
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
            //            println!("{}", i.data.display_name);
        }
    } else {
        println!("{}", String::from("NG!"));
    }

    // build terminal
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block = Block::default().title("Block 2").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;

        if let Event::Input(key) = events.next()? {
            if key == Key::Char('q') {
                break;
            }
        }
    }
    Ok(())
}
