use super::QueryBuilder;
use super::Substitute;
use std::borrow::Cow;

const SELECT_DEFAULT_TEMPLATE: &'static str = "SELECT $cnt_b$columns$cnt_e FROM $tables \
                                               $where_clause $group_by $order_by $limit $offset;";

pub struct SelectQueryBuilder<'a> {
    template:     Cow<'a, str>,
    where_clause: Option<Cow<'a, str>>,
    columns:      Option<Cow<'a, str>>,
    from_tables:  Option<Cow<'a, str>>,
    group_by:     Option<Cow<'a, str>>,
    order_by:     Option<Cow<'a, str>>,
    limit:        Option<i32>,
    offset:       Option<i32>,
    counting:     Option<bool>,
}

impl<'a> SelectQueryBuilder<'a> {
    pub fn filter<U>(mut self, where_clause: U) -> Self 
        where U: Into<Cow<'a, str>>
    {
        self.where_clause = Some(where_clause.into());
        self
    }

    pub fn columns<U>(mut self, columns: U) -> Self 
        where U: Into<Cow<'a, str>>
    {
        self.columns = Some(columns.into());
        self
    }

    pub fn from_tables<U>(mut self, tables: U) -> Self 
        where U: Into<Cow<'a, str>>
    {
        self.from_tables = Some(tables.into());
        self
    }

    pub fn group_by<U>(mut self, group_by: U) -> Self 
        where U: Into<Cow<'a, str>>
    {
        self.group_by = Some(group_by.into());
        self
    }

    pub fn order_by<U>(mut self, order_by: U) -> Self 
        where U: Into<Cow<'a, str>>
    {
        self.order_by = Some(order_by.into());
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

impl<'a> QueryBuilder<'a> for SelectQueryBuilder<'a> {
    fn default() -> Self {
        SelectQueryBuilder {
            template:     Cow::from(SELECT_DEFAULT_TEMPLATE), 
            where_clause: None,
            columns:      Some(Cow::from("*")),
            from_tables:  None,
            group_by:     None,
            order_by:     None,
            limit:        None,
            offset:       None,
            counting:     None,
        }
    }

    fn with_template<U>(template: U) -> Self 
        where U: Into<Cow<'a, str>>
    {
        let mut builder = Self::default();
        builder.template = template.into();
        builder
    }

    fn build(mut self) -> String {
        debug_assert!(self.from_tables.is_some());

        let where_clause = opt_format!(self.where_clause, "WHERE {}");
        let group_by     = opt_format!(self.group_by, "GROUP BY {}");
        let order_by     = opt_format!(self.order_by, "ORDER BY {}");
        let limit        = opt_format!(self.limit, "LIMIT {}");
        let offset       = opt_format!(self.offset, "OFFSET {}");

        let (cnt_b, cnt_e) = if self.counting.unwrap_or(false) {
            (Some(Cow::from("COUNT(")), 
             Some(Cow::from(")")))
        } else {
            (None, None)
        };

        self.template
            .substitute("$columns",      self.columns)
            .substitute("$tables",       self.from_tables)
            .substitute("$where_clause", where_clause)
            .substitute("$group_by",     group_by)
            .substitute("$order_by",     order_by)
            .substitute("$limit",        limit)
            .substitute("$offset",       offset)
            .substitute("$cnt_b",        cnt_b)    
            .substitute("$cnt_e",        cnt_e)
            .unwrap()
            .into_owned()
    }
}
