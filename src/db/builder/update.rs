use super::QueryBuilder;
use super::Substitute;
use std::borrow::Cow;
use std::char;

const UPDATE_DEFAULT_TEMPLATE: &'static str = "UPDATE $table SET $columns $where_clause;";

pub struct UpdateQueryBuilder<'a> {
    template:     Cow<'a, str>,
    where_clause: Option<Cow<'a, str>>,
    table:        Option<Cow<'a, str>>,
    columns:      Vec<Cow<'a, str>>,
}

impl<'a> UpdateQueryBuilder<'a> {
    pub fn set<U>(mut self, column: U) -> Self
        where U: Into<Cow<'a, str>>
    {
        self.columns.push(column.into());
        self
    }

    pub fn table<U>(mut self, table: U) -> Self
        where U: Into<Cow<'a, str>>
    {
        self.table = Some(table.into());
        self
    }

    pub fn filter<U>(mut self, where_clause: U) -> Self
        where U: Into<Cow<'a, str>>
    {
        self.where_clause = Some(where_clause.into());
        self
    }
}

impl<'a> QueryBuilder<'a> for UpdateQueryBuilder<'a> {
    fn default() -> Self {
        UpdateQueryBuilder {
            template: Cow::from(UPDATE_DEFAULT_TEMPLATE),
            where_clause: None,
            table: None,
            columns: Vec::new(),
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
        debug_assert!(!self.columns.is_empty());
        debug_assert!(self.table.is_some());

        let where_clause = opt_format!(self.where_clause, "WHERE {}");

        let len = self.columns.len();
        let columns = {
            let mut columns = String::with_capacity(len * 10);
            for (i, column) in self.columns.into_iter().enumerate() {
                columns.push_str(&column);
                columns.push_str("=$");
                columns.push(char::from_digit((i + 1) as u32, 10).unwrap());
                if i < len - 1 {
                    columns.push(',');
                }
            }
            Some(Cow::from(columns))
        };


        self.template
            .substitute("$table", self.table)
            .substitute("$columns", columns)
            .substitute("$where_clause", where_clause)
            .unwrap()
            .into_owned()
    }
}
