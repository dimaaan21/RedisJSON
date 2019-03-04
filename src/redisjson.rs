// RedisJSON Redis module.
//
// Translate between JSON and tree of Redis objects:
// User-provided JSON is converted to a tree. This tree is stored transparently in Redis.
// It can be operated on (e.g. INCR) and serialized back to JSON.

use serde_json::Value;
use jsonpath::Selector;

pub struct Error {
    msg: String,
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error { msg: format!("{}", e.to_string()) }
    }
}

impl From<Error> for redismodule::RedisError {
    fn from(e: Error) -> Self {
        redismodule::RedisError::String(e.msg)
    }
}

#[derive(Debug)]
pub struct RedisJSON {
    data: Value,
}

impl RedisJSON {
    pub fn from_str(data: &str) -> Result<Self, Error> {
        eprintln!("Parsing JSON from input '{}'", data);

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data)?;

        Ok(Self { data: v })
    }

    pub fn set_value(&mut self, data: &str) -> Result<(), Error> {
        eprintln!("Parsing JSON from input '{}'", data);

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data)?;

        self.data = v;

        Ok(())
    }

    pub fn to_string(&self, path: &str) -> Result<String, Error> {
        eprintln!("Serializing back to JSON");

        let s = match self.get_doc(path)? {
            Some(doc) => serde_json::to_string(&doc)?,
            None => String::new()
        };

        Ok(s)
    }

    pub fn str_len(&self, path: &str) -> Result<usize, Error> {
        let s = match self.get_doc(path)? {
            Some(doc) => doc.as_str().map_or(0, |d| d.len()),
            None => 0 // path not found
        };
        Ok(s)
    }

    pub fn get_doc(&self, path: &str) -> Result<Option<&Value>, Error> {
        // Create a JSONPath selector
        let selector = Selector::new(path).map_err(|e| Error {
            msg: format!("{}", e),
        })?;

        Ok(selector.find(&self.data).next())
    }

}