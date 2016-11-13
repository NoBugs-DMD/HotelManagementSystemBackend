macro_rules! auto_queries {
    (pub struct $table:ident {
        pub ID: i32,
        $(pub $member:ident: $member_type:ty),*
    }) => (
        auto_struct!($table id, $($member $member_type)*);
        auto_from_row!($table id, $($member)*);
        auto_queries_impls!($table $($member)*);
    );

    (pub struct $table:ident {
        $( pub $member:ident : $member_type:ty ),*
    }) => (
        auto_struct!($table, $($member $member_type)*);
        auto_from_row!($table, $($member)*);
        auto_queries_impls!($table $($member)*);
    );
}

macro_rules! auto_struct_from_row {
    (pub struct $table:ident {
        $(pub $member:ident: $member_type:ty),*
    }) => (
        auto_struct!($table, $($member $member_type)*);
        auto_from_row!($table, $($member)*);
    )
}

macro_rules! auto_struct {
    ($table:ident, $($member:ident $member_type:ty)*) => (
        #[derive(RustcEncodable, RustcDecodable, Clone, Eq, PartialEq, Debug)]
        pub struct $table {
            $(pub $member: $member_type,)*
        }

        #[allow(dead_code)]
        impl $table {
            pub fn new($($member: $member_type,)*) -> $table {
                $table {
                    $($member: $member,)*
                }
            }
        }
    );
    ($table:ident id, $($member:ident $member_type:ty)*) => (
        #[derive(RustcEncodable, RustcDecodable, Clone, Eq, PartialEq, Debug)]
        pub struct $table {
            pub ID: i32,
            $(pub $member: $member_type,)*
        }

        #[allow(dead_code)]
        impl $table {
            pub fn new($($member: $member_type,)*) -> $table {
                $table {
                    ID: -1,
                    $($member: $member,)*
                }
            }
        }
    );
}

macro_rules! auto_queries_impls {
    ($table:ident $($member:ident)*) => (
        impl Queryable for $table {
            fn select_builder() -> SelectQueryBuilder<'static> {
                SelectQueryBuilder::default()
                    .from_tables(stringify!($table))
            }
        }

        impl Deletable for $table {
            fn delete_builder() -> DeleteQueryBuilder<'static> {
                DeleteQueryBuilder::default()
                    .from_tables(stringify!($table))
            }
        }

        impl Updatable for $table {
            fn update_builder() -> UpdateQueryBuilder<'static> {
                UpdateQueryBuilder::default()
                    .table(stringify!($table))
            }
        }

        impl Insertable for $table {
            fn insert_builder() -> InsertQueryBuilder<'static> {
                InsertQueryBuilder::default()
                    .table(stringify!($table))
                $(
                    .set(stringify!($member))
                )*
            }

            fn insert_args(&self) -> Vec<&ToSql> {
                vec![
                    $(
                        &self.$member,
                    )*
                ]
            }
        }
    )
}

macro_rules! auto_from_row {
    ($table:ident, $($member:ident)*) => (
        impl<'a> From<Row<'a>> for $table {
            fn from(row: Row) -> Self {
                Self {
                    $(
                        $member: row.get(stringify!($member)),
                    )*
                }    
            }
        }
    );
    ($table:ident id, $($member:ident)*) => (
        impl<'a> From<Row<'a>> for $table {
            fn from(row: Row) -> Self {
                Self {
                    ID: row.get("ID"),
                    $(
                        $member: row.get(stringify!($member)),
                    )*
                }    
            }
        }
    )
}