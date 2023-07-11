use crate::prelude::*;
use std::error::Error;
use std::fmt;

use reqwest;

#[derive(Debug)]
pub struct Http {
    pub client: reqwest::blocking::Client,
}

impl Http {
    pub fn get(self, url: String) -> Result<Vec<serde_json::Value>> {
        let request = self.client.request(reqwest::Method::GET, &url);
        let response = request.send()?.text()?;
        let body: Vec<serde_json::Value> = serde_json::from_str(&response)?;
        Ok(body)
    }
}

#[derive(Default, Debug)]
pub struct Builder {
    pub user_agent: String,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            user_agent: "Diagora".to_string(),
        }
    }

    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    pub fn build(self) -> Result<Http> {
        let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
            .user_agent(self.user_agent)
            .build()?;
        Ok(Http { client })
    }
}

#[derive(Debug)]
struct HttpError(String);

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in Http: {}", self.0)
    }
}

impl Error for HttpError {}
