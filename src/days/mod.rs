macro_rules! import_module {
    ($mod:ident) => {
        mod $mod;
        pub use $mod::*;
    };
    ($d:literal, $p:literal) => {
        paste::paste! {
            import_module!([<day $d _part $p>]);
        }
    };
}

macro_rules! import_days {
    () => {};
    ($d:literal, $($rest:tt)*) => {
        import_module!($d, 1);
        import_module!($d, 2);
        import_days!($($rest)*);
    };
}

// normal modules, dayXX_partY
import_days!(01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13,);

// custom modules
import_module!(day01_part2_malox);

pub trait Day {
    fn solve(&self, input: &str) -> String;
}
