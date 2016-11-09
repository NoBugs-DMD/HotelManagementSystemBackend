use super::QueryBuilder;
use super::Substitute;
use std::borrow::Cow;
use std::char;

const INSERT_DEFAULT_TEMPLATE: &'static str = "INSERT INTO $table ($columns) VALUES \
                                               ($placeholders);";

pub struct InsertQueryBuilder<'a> {
    template: Cow<'a, str>,
    table:    Option<Cow<'a, str>>,
    columns:  Vec<Cow<'a, str>>,
}

impl<'a> InsertQueryBuilder<'a> {
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
}

impl<'a> QueryBuilder<'a> for InsertQueryBuilder<'a> {
    fn default() -> Self {
        InsertQueryBuilder {
            template: Cow::from(INSERT_DEFAULT_TEMPLATE),
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

        let len = self.columns.len();
        let (columns, placeholders) = {
            let mut columns = String::with_capacity(len * 10);
            let mut placeholders = String::with_capacity(len * 3);

            for (i, column) in self.columns.into_iter().enumerate() {
                columns.push_str(&column);
                placeholders.push('$');
                placeholders.push(char::from_digit((i + 1) as u32, 10).unwrap());
                if i < len - 1 {
                    columns.push(',');
                    placeholders.push(',');
                }
            }

            (Some(Cow::from(columns)), Some(Cow::from(placeholders)))
        };


        self.template
            .substitute("$table", self.table)
            .substitute("$columns", columns)
            .substitute("$placeholders", placeholders)
            .unwrap()
            .into_owned()
    }
}
