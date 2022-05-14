use std::{str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Reveal;


macro_rules! uuid_impl {
    ($uuid_type: tt) => {
        impl $uuid_type {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }
            
        }
    };
}

uuid_impl!(QuestionId);
uuid_impl!(AnswerId);
uuid_impl!(UserId);
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuestionId(Uuid);
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnswerId(Uuid);
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserId(Uuid);


impl Reveal for QuestionId {
    type Output = Uuid;
    fn reveal(&self) -> Self::Output {
        self.0
    }
}

impl Reveal for UserId {
    type Output = Uuid;
    fn reveal(&self) -> Self::Output {
        self.0
    }
}
impl Reveal for AnswerId {
    type Output = Uuid;
    fn reveal(&self) -> Self::Output {
        self.0
    }
}

impl From<Uuid> for QuestionId {
    fn from(a: Uuid) -> Self {
        Self (a)
    }
}
impl From<Uuid> for AnswerId {
    fn from(a: Uuid) -> Self {
        Self (a)
    }
}
impl From<Uuid> for UserId {
    fn from(a: Uuid) -> Self {
        Self (a)
    }
}



impl From<String> for UserId {
    fn from(u: String) -> Self {
        let id = Uuid::from_str(u.as_str())
            .expect("Error while converting ID");
        Self(id)
    }
}


impl From<String> for QuestionId {
    fn from(u: String) -> Self {
        let id = Uuid::from_str(u.as_str())
            .expect("Error while converting ID");
        Self(id)
    }
}

