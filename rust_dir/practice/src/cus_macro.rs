#[macro_export]
macro_rules! vec {
    ($($x:expr),*)=>{{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}

#[macro_export]
macro_rules! sum {
    ($($x:expr),*) => {{
        let mut total = 0;
        $(
            total += $x;
        )*
        total
    }};
}

