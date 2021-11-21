use crate::Config;
use chrono::{DateTime, NaiveDateTime, Utc};
use regex::{Captures, Regex};
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub content: String,
    pub author: String,
    pub author_email: Option<String>,
    pub create_time: DateTime<Utc>,
    pub modify_time: DateTime<Utc>,
}

impl Page {
    pub fn new(
        title: String,
        content: String,
        author: String,
        author_email: Option<String>,
        create_time: i64,
        modify_time: i64,
        url_regex_args: &Regex,
    ) -> Self {
        let create_time = DateTime::from_utc(NaiveDateTime::from_timestamp(create_time, 0), Utc);
        let modify_time = DateTime::from_utc(NaiveDateTime::from_timestamp(modify_time, 0), Utc);

        let url = url_regex_args
            .replace_all(
                &Config::read().url_patterns.post_url,
                |cap: &Captures| match &cap[0] {
                    "{slug}" => &title,
                    _ => unreachable!(),
                },
            )
            .into_owned();

        Self {
            url,
            title,
            content,
            author,
            author_email,
            create_time,
            modify_time,
        }
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.create_time.cmp(&other.create_time) {
            Ordering::Equal => self.title.cmp(&other.title),
            other => other,
        }
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
