use super::*;
use super::Substitute;
use std::borrow::Cow;

const DELETE_DEFAULT_TEMPLATE: &'static str = "DELETE FROM $from_tables $where_clause;";

pub struct DeleteQueryBuilder<'a> {
    template: Cow<'a, str>,
    from_tables: Option<Cow<'a, str>>,
    where_clause: Option<Cow<'a, str>>,
}

impl<'a> DeleteQueryBuilder<'a> {
    pub fn filter<U>(mut self, where_clause: U) -> Self
        where U: Into<Cow<'a, str>>
    {
        self.where_clause = Some(where_clause.into());
        self
    }

    pub fn from_tables<U>(mut self, tables: U) -> Self
        where U: Into<Cow<'a, str>>
    {
        self.from_tables = Some(tables.into());
        self
    }
}

impl<'a> QueryBuilder<'a> for DeleteQueryBuilder<'a> {
    fn default() -> Self {
        DeleteQueryBuilder {
            template: Cow::from(DELETE_DEFAULT_TEMPLATE),
            from_tables: None,
            where_clause: None,
        }
    }

    fn with_template<U>(template: U) -> Self
        where U: Into<Cow<'a, str>>
    {
        let mut builder = Self::default();
        builder.template = template.into();
        builder
    }

    fn build(self) -> String {
        debug_assert!(self.from_tables.is_some());
        let where_clause = opt_format!(self.where_clause, "WHERE {}");

        self.template
            .substitute("$from_tables", self.from_tables)
            .substitute("$where_clause", where_clause)
            .unwrap()
            .into_owned()
    }
}
