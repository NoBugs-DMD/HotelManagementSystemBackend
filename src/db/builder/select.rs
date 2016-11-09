use super::QueryBuilder;
use super::Substitute;

const SELECT_DEFAULT_TEMPLATE: &'static str = "SELECT $cnt_b$collumns$cnt_e FROM $tables \
                                               $where_clause $group_by $order_by $limit $offset;";

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
    counting:     Option<bool>,
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
        let where_clause = opt_format!(self.where_clause, "WHERE {}");
        let group_by     = opt_format!(self.group_by, "GROUP BY {}");
        let order_by     = opt_format!(self.order_by, "ORDER BY {}");
        let limit        = opt_format!(self.limit, "LIMIT {}");
        let offset       = opt_format!(self.offset, "OFFSET {}");

        let (cnt_b, cnt_e) = if self.counting.unwrap_or(false) {
            (Some("COUNT("), Some(")"))
        } else {
            (None, None)
        };

        self.template.take().unwrap()
            .substitute("$collumns",     opt_as_str!(self.collumns))
            .substitute("$tables",       opt_as_str!(self.from_tables))
            .substitute("$where_clause", opt_as_str!(where_clause))
            .substitute("$group_by",     opt_as_str!(group_by))
            .substitute("$order_by",     opt_as_str!(order_by))
            .substitute("$limit",        opt_as_str!(limit))
            .substitute("$offset",       opt_as_str!(offset))
            .substitute("$cnt_b",        cnt_b)    
            .substitute("$cnt_e",        cnt_e)
            .unwrap()
    }
}
