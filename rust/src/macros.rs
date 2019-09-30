extern crate paste;

#[macro_export]
macro_rules! pad { ( $x:ident ) => { kb!(NO) } }

macro_rules! keycode {
    (,) => { KC_COMMA };
    (.) => { KC_DOT };
    (-) => { KC_MINUS };
    (=) => { KC_EQUAL };
    (;) => { KC_SCLN };
    (/) => { KC_SLSH };
    (!) => { KC_EXLM };
    (@) => { KC_AT };
    (#) => { KC_HASH };
    ($) => { KC_DLR };
    (%) => { KC_PERC };
    (^) => { KC_CIRC };
    (&) => { KC_AMPR };
    (*) => { KC_ASTR };
    (_) => { KC_UNDS };
    (+) => { KC_PLUS };
    ('[') => { KC_LBRC };
    (']') => { KC_RBRC };
    ('(') => { KC_LPRN };
    (')') => { KC_RPRN };
    (?) => { KC_QUEST };
    ('⏎') => { KC_ENTER };
    ('↑') => { KC_UP };
    ('→') => { KC_RIGHT };
    ('↓') => { KC_DOWN };
    ('←') => { KC_LEFT };
    ('↹') => { KC_TAB };
    ('`') => { KC_GRV };
    ('🔊') => { KC_VOLU };
    ('🔉') => { KC_VOLD };
    ('🔇') => { KC_MUTE };
    ([ ]) => { KC__TODO_ };
    (xxx) => { kb!(NO) };
    (xxxx) => { kb!(NO) };
    (xxxxx) => { kb!(NO) };
    (xxxxxx) => { kb!(NO) };
    (xxxxxxx) => { kb!(NO) };
    (xxxxxxxx) => { kb!(NO) };

    ([$key:ident&LT{$layernu:literal}]) => {
        LT!($layernu, keycode!($key))
    };
    ([$key:literal&LT{$layernu:literal}]) => {
        LT!($layernu, keycode!($key))
    };
    ([MO{$layernu:literal}]) => {
        MO!($layernu)
    };
    ($key:literal) => {
        paste::expr!{ [<KC_ $key>] }
    };
    ($key:ident) => {
        paste::expr!{ [<KC_ $key>] }
    };
    ($key:expr) => {
        $key
    };
    ($key:ident&LT($layernu:literal)) => {
        LT!($layernu, paste::expr!{ [<KC_ $key>] })
    };
}

#[macro_export]
macro_rules! r {
    ( $( $key:tt )|+ ) => {
        [ $( keycode!($key), )* ]
    }
}

#[macro_export]
macro_rules! layer {
    ( $( $row:expr ),+ $(,)?) => {
        [ $( $row, )* ]
    }
}

#[macro_export]
macro_rules! keymaps {
    (
        rows => $rows: ident,
        cols => $cols: ident,
        layer_cnt => $layer_cnt: literal,
        $( $layer:expr ),+ $(,)?
    ) => {
        #[no_mangle]
        static keymaps: [[[u16; $cols]; $rows]; $layer_cnt] = [
            $( $layer, )*
        ];
    }
}
