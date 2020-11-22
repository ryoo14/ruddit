use serde::Deserialize;

#[derive(Deserialize)]
pub struct RedditResponseData {
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct BasicThing<T> {
    pub kind: String,
    pub data: T,
}

#[derive(Deserialize)]
pub struct Listing<T> {
    pub modhash: Option<String>,
    pub dist: Option<i32>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub children: Vec<T>,
}

// subscriber
pub type RedditResponse = BasicListing<RedditResponseData>;
pub type BasicListing<T> = BasicThing<Listing<BasicThing<T>>>;
