use uuid::Uuid;
use std::error;
use std::fmt;

#[derive(Clone)]
pub struct Profile {
    uuid: Option<Uuid>,
    name: Option<String>,
    pub legacy: bool,
}

#[derive(Debug, Clone)]
pub struct ProfileError(String);

impl error::Error for ProfileError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Profile {
    pub fn new(uuid: Option<Uuid>, name: Option<String>) -> Result<Profile, ProfileError> {
        if uuid.is_none() && name.is_none() {
            Err(ProfileError(String::from("Both UUID & name cannot be empty!")))
        } else {
            Ok(Profile {
                uuid,
                name,
                legacy: false
            })
        }
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Profile {
            uuid: Some(uuid),
            name: None,
            legacy: false,
        }
    }

    pub fn from_name<S: Into<String>>(name: S) -> Self {
        Profile {
            uuid: None,
            name: Some(name.into()),
            legacy: false,
        }
    }

    pub fn from_uuid_and_name(uuid: Uuid, name: String) -> Self {
        Profile {
            uuid: Some(uuid),
            name: Some(name),
            legacy: false,
        }
    }

    pub fn uuid(&self) -> Option<&Uuid> {
        self.uuid.as_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}