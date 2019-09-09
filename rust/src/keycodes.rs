#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate paste;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_export]
macro_rules! qkc {
    ( $x:ident ) => {
        paste::expr! { [<quantum_keycodes_ $x>] } as u16
    };
}

#[macro_export]
macro_rules! kb {
    ( $x:ident ) => {
        paste::expr! { [<hid_keyboard_keypad_usage_KC_ $x>] } as u16
    };
    ( $x:literal ) => {
        paste::expr! { [<hid_keyboard_keypad_usage_KC_ $x>] } as u16
    };
}

#[macro_export]
macro_rules! fk {
    ( $x:ident ) => {
        paste::expr! { [<internal_special_keycodes_KC_ $x>] } as u16
    };
}

// #[macro_export]
// macro_rules! ms {
//     ( $x:ident ) => {
//         paste::expr! { [<mouse_keys_KC_MS_ $x>] } as u16
//     };
// }

#[macro_export]
macro_rules! LSFT {
    ( $kc:expr ) => {
        qkc!(QK_LSFT) | $kc
    }
}

#[macro_export]
macro_rules! MO {
    ( $layer:literal ) => {
        qkc!(QK_MOMENTARY) | ($layer & 0xFF)
    }
}

#[macro_export]
macro_rules! LT {
    ( $layer:literal, $kc:expr ) => {
        qkc!(QK_LAYER_TAP) | (($layer & 0xF) << 8) | ($kc & 0xFF)
    }
}

pub const KC_A: u16 = kb!(A);
pub const KC_B: u16 = kb!(B);
pub const KC_C: u16 = kb!(C);
pub const KC_D: u16 = kb!(D);
pub const KC_E: u16 = kb!(E);
pub const KC_F: u16 = kb!(F);
pub const KC_G: u16 = kb!(G);
pub const KC_H: u16 = kb!(H);
pub const KC_I: u16 = kb!(I);
pub const KC_J: u16 = kb!(J);
pub const KC_K: u16 = kb!(K);
pub const KC_L: u16 = kb!(L);
pub const KC_M: u16 = kb!(M);
pub const KC_N: u16 = kb!(N);
pub const KC_O: u16 = kb!(O);
pub const KC_P: u16 = kb!(P);
pub const KC_Q: u16 = kb!(Q);
pub const KC_R: u16 = kb!(R);
pub const KC_S: u16 = kb!(S);
pub const KC_T: u16 = kb!(T);
pub const KC_U: u16 = kb!(U);
pub const KC_V: u16 = kb!(V);
pub const KC_W: u16 = kb!(W);
pub const KC_X: u16 = kb!(X);
pub const KC_Y: u16 = kb!(Y);
pub const KC_Z: u16 = kb!(Z);

pub const KC_F1: u16 = kb!(F1);
pub const KC_F2: u16 = kb!(F2);
pub const KC_F3: u16 = kb!(F3);
pub const KC_F4: u16 = kb!(F4);
pub const KC_F5: u16 = kb!(F5);
pub const KC_F6: u16 = kb!(F6);
pub const KC_F7: u16 = kb!(F7);
pub const KC_F8: u16 = kb!(F8);
pub const KC_F9: u16 = kb!(F9);
pub const KC_F10: u16 = kb!(F10);
pub const KC_F11: u16 = kb!(F11);
pub const KC_F12: u16 = kb!(F12);

pub const KC_1: u16 = kb!(1);
pub const KC_2: u16 = kb!(2);
pub const KC_3: u16 = kb!(3);
pub const KC_4: u16 = kb!(4);
pub const KC_5: u16 = kb!(5);
pub const KC_6: u16 = kb!(6);
pub const KC_7: u16 = kb!(7);
pub const KC_8: u16 = kb!(8);
pub const KC_9: u16 = kb!(9);
pub const KC_0: u16 = kb!(0);
pub const KC_MINUS: u16 = kb!(MINUS);
pub const KC_MINS: u16 = KC_MINUS;
pub const KC_EQUAL: u16 = kb!(EQUAL);

pub const KC_EXLM: u16 = LSFT!(KC_1);     // !
pub const KC_AT  : u16 = LSFT!(KC_2);     // @
pub const KC_HASH: u16 = LSFT!(KC_3);     // #
pub const KC_DLR : u16 = LSFT!(KC_4);     // $
pub const KC_PERC: u16 = LSFT!(KC_5);     // %
pub const KC_CIRC: u16 = LSFT!(KC_6);     // ^
pub const KC_AMPR: u16 = LSFT!(KC_7);     // &
pub const KC_ASTR: u16 = LSFT!(KC_8);     // *
pub const KC_LPRN: u16 = LSFT!(KC_9);     // (
pub const KC_RPRN: u16 = LSFT!(KC_0);     // )
pub const KC_UNDS: u16 = LSFT!(KC_MINS);  // _
pub const KC_PLUS: u16 = LSFT!(KC_EQUAL); // +

pub const KC_UP: u16 = kb!(UP);
pub const KC_DOWN: u16 = kb!(DOWN);
pub const KC_LEFT: u16 = kb!(LEFT);
pub const KC_RIGHT: u16 = kb!(RIGHT);
pub const KC_SPACE: u16 = kb!(SPACE);

pub const KC_TG_NKRO: u16 = qkc!(MAGIC_TOGGLE_NKRO);  //Toggle 6KRO / NKRO mode
pub const KC__TODO_: u16 = KC_TRNS as u16;
pub const KC________: u16 = KC__TODO_;
pub const KC_PGDN: u16 = kb!(PGDOWN);
pub const KC_VOLU: u16 = fk!(AUDIO_VOL_UP);
pub const KC_VOLD: u16 = fk!(AUDIO_VOL_DOWN);
pub const KC_PAUS: u16 = kb!(PAUSE);
pub const KC_SLCK: u16 = kb!(SCROLLLOCK);
pub const KC_PSCR: u16 = kb!(PSCREEN);
pub const KC_MUTE: u16 = fk!(AUDIO_MUTE);
pub const KC_ESC: u16 = kb!(ESCAPE);
pub const KC_LCTL: u16 = kb!(LCTRL);
pub const KC_RSFT: u16 = kb!(RSHIFT);
pub const KC_SLSH: u16 = kb!(SLASH);
pub const KC_QUEST: u16 = LSFT!(KC_SLSH);
pub const KC_LSFT: u16 = kb!(LSHIFT);
pub const KC_SCLN: u16 = kb!(SCOLON);
pub const KC_CAPS: u16 = kb!(CAPSLOCK);
pub const KC_BSLS: u16 = kb!(BSLASH);
pub const KC_RBRC: u16 = kb!(RBRACKET);
pub const KC_LBRC: u16 = kb!(LBRACKET);
pub const KC_DEL: u16 = kb!(DELETE);
pub const KC_BSPC: u16 = kb!(BSPACE);
pub const KC_GRV: u16 = kb!(GRAVE);
pub const KC_TAB: u16 = kb!(TAB);
pub const KC_HOME: u16 = kb!(HOME);
pub const KC_DOT: u16 = kb!(DOT);
pub const KC_COMMA: u16 = kb!(COMMA);
pub const KC_ENTER: u16 = kb!(ENTER);
pub const KC_PGUP: u16 = kb!(PGUP);
pub const KC_QUOTE: u16 = kb!(QUOTE);
pub const KC_LALT: u16 = kb!(LALT);
pub const KC_RALT: u16 = kb!(RALT);
pub const KC_LGUI: u16 = kb!(LGUI);
pub const KC_RGUI: u16 = kb!(RGUI);
pub const KC_END: u16 = kb!(END);
