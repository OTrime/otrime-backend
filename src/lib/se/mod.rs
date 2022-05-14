use crate::{http::models::QuestionWithName, IntoInner};

pub struct Se;
#[derive(Clone, Debug)]
pub struct SearchField(String);

impl From<String> for SearchField {
    fn from(a: String) -> Self {
        Self(a)
    }
}
impl IntoInner for SearchField {
    type Output = String;
    fn into_inner(self) -> Self::Output {
        self.0
    }
}

impl<'a> Se {
    pub fn sort(
        search_field: SearchField,
        questions: &'a Vec<QuestionWithName>,
    ) -> Vec<&'a QuestionWithName> {
        let term = search_field.into_inner();
        let q = questions
            .iter()
            .filter(|q| q.title.contains(&term))
            .collect::<Vec<_>>();

        q
    }
}
