use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CastRoot {
    pub result: Result,
    pub next: Option<Next>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Result {
    pub casts: Vec<Cast>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cast {
    pub hash: String,
    #[serde(rename="threadHash")]
    pub thread_hash: String,
    pub author: Author,
    pub text: String,
    pub timestamp: i64,
    pub replies: Replies,
    pub reactions: Reactions,
    pub recasts: Recasts,
    pub watches: Watches,
    #[serde(rename="viewerContext")]
    pub viewer_context: ViewerContext
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub fid: i64,
    pub username: String,
    #[serde(rename="displayName")]
    pub display_name: String,
    pub pfp: PFP,
    pub profile: Profile,
    #[serde(rename="followerCount")]
    pub follower_count: i64,
    #[serde(rename="followingCount")]
    pub following_count: i64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PFP {
    pub url: String,
    pub verified: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub bio: Bio
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bio {
    pub text: String,
    pub mentions: Option<Vec<String>>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Replies {

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reactions {

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recasts {

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Watches {

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewerContext {

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Next {
    pub cursor: Option<String>,
}
