use super::QueryBuilder;
use super::substitute;

const SELECT_DEFAULT_TEMPLATE: &'static str = 
    "SELECT $cnt_b$collumns$cnt_e FROM $tables $where_clause $group_by $order_by $limit $offset;";

#[derive(Default)]
pub struct SelectQueryBuilder {
    template:     Option<String>,
    where_clause: Option<String>,
    collumns:     Option<String>,
    from_tables:  Option<String>,    
    group_by:     Option<String>,
    order_by:     Option<String>,
    limit:        Option<i32>,
    offset:       Option<i32>,
    counting:     Option<bool>
}

impl SelectQueryBuilder {
    pub fn filter(mut self, where_clause: &str) -> Self {
        self.where_clause = Some(where_clause.to_owned());
        self
    }
    
    pub fn columns(mut self, collumns: &str) -> Self {
        self.collumns = Some(collumns.to_owned());
        self
    }

    pub fn from_tables(mut self, tables: &str) -> Self {
        self.from_tables = Some(tables.to_owned());
        self
    }

    pub fn group_by(mut self, group_by: &str) -> Self {
        self.group_by = Some(group_by.to_owned());
        self
    }

    pub fn order_by(mut self, order_by: &str) -> Self {
        self.order_by = Some(order_by.to_owned());
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn counting(mut self, counting: bool) -> Self {
        self.counting = Some(counting);
        self
    }
}

impl QueryBuilder for SelectQueryBuilder {
    fn new() -> Self {
        let mut builder = Self::default();
        builder.collumns = Some(String::from("*"));
        builder.template = Some(SELECT_DEFAULT_TEMPLATE.to_owned());
        builder
    }

    fn with_template(template: String) -> Self {
        let mut builder = Self::default();
        builder.template = Some(template);
        builder
    }

    fn build(mut self) -> String {
        let template = self.template.take().unwrap();

        let template = substitute("$collumns", template, self.collumns.as_ref()).unwrap();
        let template = substitute("$tables", template, self.from_tables.as_ref()).unwrap();
        let template = substitute("$where_clause", template, self.where_clause.map(|w| format!("WHERE {}", w)).as_ref()).unwrap();
        let template = substitute("$group_by", template, self.group_by.map(|g| format!("GROUP BY {}", g)).as_ref()).unwrap();
        let template = substitute("$order_by", template, self.order_by.map(|o| format!("ORDER BY {}", o)).as_ref()).unwrap();
        let template = substitute("$limit", template, self.limit.map(|l| format!("LIMIT {}", l)).as_ref()).unwrap();
        let template = substitute("$offset", template, self.offset.map(|o| format!("OFFSET {}", o)).as_ref()).unwrap();
        
        let is_counting = self.counting.unwrap_or(false);
        let cnt_b = if is_counting { Some(String::from("COUNT(")) } else { None };
        let cnt_e = if is_counting { Some(String::from(")")) } else { None };
        
        let template = substitute("$cnt_b", template, cnt_b.as_ref()).unwrap();
        let template = substitute("$cnt_e", template, cnt_e.as_ref()).unwrap();

        template
    }    
}
