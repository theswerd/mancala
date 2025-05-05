macro_rules! import_and_extract {
    [$($m:ident,)*] => {
        $(
            mod $m;
            pub use $m::*;
        )*
    };
}

import_and_extract![
    bank_bird_1,
    bank_bird_2,
];
