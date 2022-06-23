pub struct Request {
    base_url: &'static str,
}

impl Request {
    pub fn build_url(&mut self, path: Option<&str>) -> String {
        match path {
            None => String::from(self.base_url),
            Some(endpoint) => {
                let mut base: String = self.base_url.clone().to_owned();
                base.push_str("/");
                base.push_str(endpoint);
                base
            }
        }
    }

    pub fn get(&mut self, path: Option<&str>) -> String {
        self.build_url(path)
    }

    pub fn post(&mut self, path: Option<&str>) -> String {
        self.build_url(path)
    }
}

pub fn test() {
    let mut a = Request {
        base_url: "https://jsonplaceholder.typicode.com",
    };
    {
        let response = a.post(None);
        println!("{:#?}", response);
    }
    {
        let response = a.post(Some("subpath/endpoint"));
        println!("{:#?}", response);
    }
}
