use std::error;
use std::fmt;
use regex::Regex;

lazy_static! {
    static ref IS_NAMESPACE: Regex = Regex::new("^[a-z0-9_.-]+$").unwrap();
    static ref IS_PATH: Regex = Regex::new("^[a-z0-9/._-]+$").unwrap();
}

#[derive(Debug, Clone)]
pub struct ResourceLocationError(String);

impl error::Error for ResourceLocationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for ResourceLocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[derive(PartialEq, Debug)]
pub struct ResourceLocation {
    namespace: String,
    path: String,
}

impl ResourceLocation {
    pub fn new(namespace: &str, path: &str) -> ResourceLocation {
        ResourceLocation {
            namespace: String::from(namespace),
            path: String::from(path),
        }
    }

    pub fn parse(s: &str) -> Result<ResourceLocation, ResourceLocationError> {
        let (namespace, path) = if s.contains(":") {
            let namespace;
            let path;

            if s.starts_with(":") {
                namespace = String::from("minecraft");
                path = String::from(&s[1..]);
            } else {
                let i = s.find(":").unwrap();

                namespace = String::from(&s[..i]);
                path = String::from(&s[(i + 1)..]);
            }

            (namespace, path)
        } else {
            let namespace = String::from("minecraft");
            let path = String::from(s);

            (namespace, path)
        };

        if !Self::is_valid_namespace(&namespace) {
            Err(ResourceLocationError(String::from(r"Namespace must only contain: [a-z0-9_.-]")))
        } else if !Self::is_valid_path(&path) {
            Err(ResourceLocationError(String::from(r"Path must only contain: [a-z0-9/._-]")))
        } else {
            Ok(ResourceLocation { namespace, path })
        }
    }

    pub fn is_valid_namespace(s: &str) -> bool {
        IS_NAMESPACE.is_match(s)
    }

    pub fn is_valid_path(s: &str) -> bool {
        IS_PATH.is_match(s)
    }
}

#[cfg(test)]
mod test {
    use super::ResourceLocation;

    #[test]
    fn parse() {
        println!("{}", ResourceLocation::parse("minecraft@test1234").unwrap_err());
    }
}