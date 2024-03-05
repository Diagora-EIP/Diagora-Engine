use crate::prelude::*;
use reqwest;

/// Overide of reqwest lib that let me more liberty and clarity
#[derive(Debug, Clone)]
pub struct Http {
    pub client: reqwest::blocking::Client,
}

/// Implementation of function that will be usefull
impl Http {
    ///
    /// Function that will do a get request of an url and will return it's result
    ///
    /// # Arguments
    ///
    /// * `url` - Url that you want to do a request
    ///
    /// # Return
    ///
    /// * String - the  result of the request
    ///
    pub fn get(self, url: String) -> Result<String> {
        let request = self.client.request(reqwest::Method::GET, &url);
        let response = request.send()?.text()?;
        Ok(response)
    }
}

/// Builder of the Request Client
#[derive(Debug)]
pub struct Builder {
    pub user_agent: String,
}

impl Builder {
    /// Init of the builder
    pub fn new() -> Self {
        Builder {
            user_agent: "Diagora".to_string(),
        }
    }

    /// Possibility to modify the user_agent
    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    /// Built the Reqwest client
    ///
    /// # Return
    ///
    /// * Http - Return the overide of reqwest
    pub fn build(self) -> Result<Http> {
        let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
            .user_agent(self.user_agent)
            .build()?;
        Ok(Http { client })
    }
}
