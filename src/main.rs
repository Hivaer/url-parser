use std::collections::HashMap;


enum UrlParser {
    Protocol,
    Domain,
    Port,
    Path,
    Query,
    Fragment
}

fn main() {
    let mut url = Url::new();
    println!("{:?}", url.from_raw("https://abc.example.com:443/path/:wildcard/index.html?foo=bar&biz=fiz#fragment"));
}

#[derive(Debug)]
struct Url {
    protocol: String,
    sub_domain: Vec<String>,
    domain: String,
    top_level_domain: String,
    port: String,
    path: Vec<String>,
    query: HashMap<String, String>,
    fragment: String
}

impl Url {

    pub fn new() -> Url {
        Url {
            protocol: String::new(),
            sub_domain: vec![],
            domain: String::new(),
            top_level_domain: String::new(),
            port: String::new(),
            path: vec![],
            query: HashMap::new(),
            fragment: String::new()
        }
    }

    pub fn from_raw(&mut self, str: &str) -> &Self {

        let mut state = UrlParser::Protocol;
        
        let mut parameter = String::new();

        let mut skip_index = 0;

        for (i, current_byte) in str.as_bytes().iter().enumerate() {

           if i < skip_index {
                continue;
            }

            let current = *current_byte as char;
            let next_1 = if i + 1 < str.as_bytes().len() { str.as_bytes()[i + 1] as char } else { '\0' };
            let next_2 = if i + 2 < str.as_bytes().len() { str.as_bytes()[i + 2] as char } else { '\0' };

            match state {
                UrlParser::Protocol => match current {
                    ':' if next_1 == '/' && next_2 == '/' => {
                        state = UrlParser::Domain;
                        Url::assign_value(&mut self.protocol, &mut parameter);
                        skip_index = i + 3;
                    },
                    '.' | ':' | '?' | '/' => {
                        state = UrlParser::Domain;
                        Url::assign_value(&mut self.domain, &mut parameter);
                    }
                    _ => parameter.push(current)
                },
                UrlParser::Domain => match current {
                    '.' => {
                        if self.domain.is_empty() {
                            println!("Set Domain to: {parameter}");
                            Url::assign_value(&mut self.domain, &mut parameter);
                        } else {
                            println!("Set subdomain to: {}", self.domain);
                            println!("Set Domain to: {parameter}");
                            Url::assign_vector_value(&mut self.sub_domain, &mut self.domain);
                            Url::assign_value(&mut self.domain, &mut parameter);
                        }
                        parameter.clear();
                    },
                    ':' => {
                        if !self.domain.is_empty() {
                            println!("Set TLD to: {parameter}");

                            Url::assign_value(&mut self.top_level_domain, &mut parameter);
                            state = UrlParser::Port;
                        }
                    }
                    _ => {
                        parameter.push(current);

                        if i == str.as_bytes().len()-1 {
                            if !self.domain.is_empty() {
                                println!("Set TLD to: {parameter}");
                                Url::assign_value(&mut self.top_level_domain, &mut parameter);
                            } else {
                                println!("Set Domain to: {parameter}");
                                Url::assign_value(&mut self.domain, &mut parameter);
                            }
                        }
                    }
                },
                UrlParser::Port => match current {
                    '/' => {
                        println!("Set port to: {parameter}");
                        Url::assign_value(&mut self.port, &mut parameter);
                        state = UrlParser::Path;
                    },
                    '?' => {
                        println!("Set port to: {parameter}");
                        Url::assign_value(&mut self.port, &mut parameter);
                        state = UrlParser::Query;
                    },
                    '#' => {
                        println!("Set port to: {parameter}");
                        Url::assign_value(&mut self.port, &mut parameter);
                        state = UrlParser::Fragment;
                    }
                    _ => {
                        parameter.push(current);

                        if i == str.as_bytes().len()-1 {
                            println!("Set port to: {parameter}");
                            Url::assign_value(&mut self.port, &mut parameter);
                        }
                    }
                },
                UrlParser::Path => match current {
                    '/' => {
                        println!("add to path: {parameter}");
                        Url::assign_vector_value(&mut self.path, &mut parameter);
                    },
                    '?' => {
                        println!("add to path: {parameter}");
                        Url::assign_vector_value(&mut self.path, &mut parameter);
                        state = UrlParser::Query;
                    },
                    '#' => {
                        println!("add to path: {parameter}");
                        Url::assign_vector_value(&mut self.path, &mut parameter);
                        state = UrlParser::Fragment;
                    }
                    _ => parameter.push(current)
                },
                UrlParser::Query => match current {
                    '&' | '#' => {
                        println!("add to query: {parameter}");
                        self.assign_query_values( &mut parameter);

                        if current == '#' {
                            state = UrlParser::Fragment;
                        }
                    }
                    _ => {
                        parameter.push(current);

                        if i == str.as_bytes().len()-1 {
                            println!("add to query: {parameter}");
                            self.assign_query_values(&mut parameter);
                        }
                    }
                },
                UrlParser::Fragment => match current {
                    _ => {
                        parameter.push(current);
                        if i == str.as_bytes().len()-1 {
                            Url::assign_value(&mut self.fragment, &mut parameter);
                        }
                    }
                },
            }
        }

        self
    }

    fn assign_query_values(&mut self, parameter: &mut String) {
        let mut key = String::new();
        let mut value = String::new();

        let mut is_key = true;
        for c in parameter.chars().into_iter() {
            if c != '=' && is_key {
                key.push(c)
            } else if c != '=' {
                value.push(c);
            } else {
                is_key = false;
            }
        }

        self.query.insert(key, value);

        parameter.clear();
    }

    fn assign_value(value: &mut String, parameter: &mut String) {
        *value = parameter.clone();
        parameter.clear();
    }

    fn assign_vector_value(value: &mut Vec<String>, parameter: &mut String) {
        value.push(parameter.clone());
        parameter.clear();
    }
}


