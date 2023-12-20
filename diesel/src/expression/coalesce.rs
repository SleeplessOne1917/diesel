use diesel::{
    dsl::sql,
    sql_types::{Bool, Nullable},
};

#[macro_export]
macro_rules! coalesce {
    (@arg_def count:expr => [$_arg:expr]) => {
        arg$count: ST
    };
    (@arg_def count:expr => [$_arg:expr, $($rest_args:tt)*]) => {
        arg$count: ST, $crate::expression::coalesce!(@arg_def ($count + 1) => [$($rest_args)*])
    };
    ($_arg1:expr, $_arg2:expr, $($rest_args:tt)*) => {
        $crate::sql_function! {
            #[sql_name = "COALESCE"]
            fn coalesce_inner<ST: $crate::sql_types::SqlType + $crate::sql_types::SingleValue>(arg1: ST, arg2: ST, $($crate::expression::coalesce!(@arg_def 3 => [$($rest_args)*]))*) -> $crate::sql_types::Nullable<ST>;
        };
    };
}

pub fn foo() {
    coalesce!(sql::<Nullable<Bool>>("NULL"), sql::<Nullable<Bool>>("true"),);
}
