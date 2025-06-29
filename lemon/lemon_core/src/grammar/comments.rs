use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Comment {
    LineComment,
    BlockCommentOpen,
    BlockCommentClose,
}
impl Comment {
    pub fn from_str(s: &str) -> Option<Comment> {
        match s {
            "//" => Some(Comment::LineComment),
            "/*" => Some(Comment::BlockCommentOpen),
            "*/" => Some(Comment::BlockCommentClose),
            _ => None,
        }
    }
}
