use std::fs;
use std::fmt;

pub mod Os {
    pub struct Os {
        name: String,
        version: String
    }
    
    impl Os {
        pub fn build(path: &str) -> Result<Self, &'static str>
        {
            let file_content = fs::read_to_string(path).unwrap_or_else(|err| {
                format!("Error while getting os info : {}", err)
            });
            Ok(Os {
                name : "lol",
                version : "0.01"
            })
        }
    }
}