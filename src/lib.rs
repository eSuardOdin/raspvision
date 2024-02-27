use std::fs;
// use std::fmt;

#[derive(Debug)]
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
        let mut f_name = String::from("");
        let mut f_version = String::from("");
        for line in file_content.lines()
        {
            // Get os name
            if !line.contains("PRETTY") && line.contains("NAME")
            {
                // If enclosed in ""
                if let Some(start) = line.find('"') {
                    if let Some(end) = line.rfind('"') {
                        f_name = line[start+1 .. end].to_string();
                    }
                }
            }

            // Get os version
            if line.contains("VERSION_ID")
            {
                if let Some(start) = line.find('"') {
                    if let Some(end) = line.rfind('"') {
                        f_version = line[start+1 .. end].to_string();
                    }
                }
            }
        }
        let mut os = Os {
            name : f_name,
            version : f_version
        };

        Ok(os)
    }
}