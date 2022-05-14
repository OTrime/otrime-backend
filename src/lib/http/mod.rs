pub mod ids;
pub mod models;
pub mod web;

use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};

use crate::errors::ApiError;

pub trait Reveal {
    type Output;
    fn reveal(&self) -> Self::Output;
}

macro_rules! create_new_type {
    ($name: ident, $insider: tt) => {
        #[derive(Clone, Serialize, Deserialize, Debug)]
        pub struct $name($insider);
    };
}

create_new_type!(Email, String);
create_new_type!(Password, String);
impl Reveal for Email {
    type Output = String;
    fn reveal(&self) -> Self::Output {
        self.0.to_owned()
    }
}
impl Reveal for Password {
    type Output = String;
    fn reveal(&self) -> Self::Output {
        self.0.to_owned()
    }
}

impl Password {
    pub fn new(password: String) -> Self {
        Self(password)
    }
    pub fn hash(&self) -> Result<Self, ApiError> {
        let hashed = hash(&self.0, DEFAULT_COST)?;
        Ok(Password(hashed))
    }

    pub fn pverify(&self, password: &str) -> Result<bool, ApiError> {
        let is_password_correct = bcrypt::verify(password, &self.0)?;
        Ok(is_password_correct)
    }
}
