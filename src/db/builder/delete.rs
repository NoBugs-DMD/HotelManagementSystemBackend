use super::*;
use super::substitute;

const DELETE_DEFAULT_TEMPLATE: &'static str = 
    "DELETE FROM $from_tables $where_clause;";


#[derive(Default)]
pub struct DeleteQueryBuilder {
    template:     Option<String>,
    from_tables:  Option<String>,
    where_clause: Option<String>
}

impl DeleteQueryBuilder {
    pub fn filter(mut self, where_clause: &str) -> Self {
        self.where_clause = Some(where_clause.to_owned());
        self
    }

    pub fn from_tables(mut self, tables: &str) -> Self {
        self.from_tables = Some(tables.to_owned());
        self
    }
}

impl QueryBuilder for DeleteQueryBuilder {
    fn new() -> Self {
        let mut builder = Self::default();
        builder.template = Some(DELETE_DEFAULT_TEMPLATE.to_owned());
        builder
    }

    fn with_template(template: String) -> Self {
        let mut builder = Self::default();
        builder.template = Some(template);
        builder
    }

    fn build(mut self) -> String {
        let template = self.template.take().unwrap();

        let template = substitute("$tables", template, self.from_tables.as_ref()).unwrap();
        let template = substitute("$where_clause", template, self.where_clause.map(|w| format!("WHERE {}", w)).as_ref()).unwrap();
        
        template
    }    
}
