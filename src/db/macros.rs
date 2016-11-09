macro_rules! auto_queries {
    (pub struct $table:ident {
        pub ID: i32,
        $(pub $member:ident: $member_type:ty),*
    }) => (
        #[derive(RustcEncodable, RustcDecodable, Clone, Debug)]
        pub struct $table {
            pub ID: i32,
            $(pub $member: $member_type,)*
        }

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

        auto_queries_impls!($table $($member)*);
    );

    (pub struct $table:ident {
        $( pub $member:ident : $member_type:ty ),*
    }) => (
        #[derive(RustcEncodable, RustcDecodable, Clone, Debug)]
        pub struct $table {
            $(pub $member: $member_type,)*
        }

        impl<'a> From<Row<'a>> for $table {
            fn from(row: Row) -> Self {
                Self {
                    $(
                        $member: row.get(stringify!($member)),
                    )*
                }    
            }
        }

        auto_queries_impls!($table $($member)*);
    );
}

macro_rules! auto_queries_impls {
    ($table:ident $($member:ident)*) => (
        impl Insertable for $table {
            fn insert_query() -> String {
                concat!("INSERT INTO ", stringify!($table), "(").to_owned() 
                    + &members_list!($($member)*) + ")"
                    + " VALUES(" + &substitution_list!($($member)*) + ")"
            }
        }

        impl Queryable for $table {
            fn select_builder() -> SelectQueryBuilder {
                SelectQueryBuilder::new()
                    .from_tables(stringify!($table))
            }
        }

        impl Deletable for $table {
            fn delete_builder() -> DeleteQueryBuilder {
                DeleteQueryBuilder::new()
                    .from_tables(stringify!($table))
            }
        }
    )
}

macro_rules! members_list {
    ($($member:ident)*) => ({ 
        let cnt = count!($($member)*);
        let mut s = String::new(); 
        let mut i = 0;
        $(
            s.push_str(
                &format!("{}{}", stringify!($member), {
                    i += 1; if i < cnt { ", " } else { "" }
                })
            );
        )*
        s    
    })
}

macro_rules! substitution_list {
    ($($member:ident)*) => ({ 
        let cnt = count!($($member)*);
        let mut s = String::new(); 
        for i in 1...cnt { s += &format!("${}{}", i, {
            if i < cnt { ", " } else { "" }
        });} 
        s 
    })
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}