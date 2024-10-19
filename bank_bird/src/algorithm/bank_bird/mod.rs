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
    bank_bird_3,
    bank_bird_4,
];
