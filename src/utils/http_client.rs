use reqwest::{Client, Error};
use serde::Deserialize;

pub struct Request {
    pub base_url: &'static str,
}

impl Request {
    pub fn build_url(&mut self, path: Option<&str>) -> String {
        match path {
            None => String::from(self.base_url),
            Some(endpoint) => {
                let mut base: String = (*self.base_url).to_string();
                base.push('/');
                base.push_str(endpoint);
                base
            }
        }
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(
        &mut self,
        path: Option<&str>,
    ) -> Result<T, Error> {
        let url: String = self.build_url(path);
        let client: Client = Client::new();
        let response: T = client.get(url).send().await?.json::<T>().await?;
        Ok(response)
    }

    pub async fn post<T: for<'de> Deserialize<'de>>(
        &mut self,
        path: Option<&str>,
    ) -> Result<T, Error> {
        let url: String = self.build_url(path);
        let client: Client = Client::new();
        let response: T = client.post(url).send().await?.json::<T>().await?;
        Ok(response)
    }
}
