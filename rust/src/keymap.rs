use super::keycodes::*;

/* from: keyboards/massdrop/alt/keymaps/houqp/keymap.c */
#[allow(non_camel_case_types)]
pub enum LedKeycodes {
    BRI = qkc!(SAFE_RANGE) as isize, //LED Brightness Increase
    BRD,                             //LED Brightness Decrease
    EDG_I,                           //LED Edge Brightness Increase
    EDG_D,                           //LED Edge Brightness Decrease
    EDG_M,                           //LED Edge lighting mode
    PTN,                             //LED Pattern Select Next
    PTP,                             //LED Pattern Select Previous
    PSI,                             //LED Pattern Speed Increase
    PSD,                             //LED Pattern Speed Decrease
    T_MD,                            //LED Toggle Mode
    T_ONF,                           //LED Toggle On / Off
    ON,                              //LED On
    OFF,                             //LED Off
    T_BR,                            //LED Toggle Breath Effect
    T_PTD,                           //LED Toggle Scrolling Pattern Direction and effect
}

macro_rules! led {
    ( $x:ident ) => {
        LedKeycodes::$x as u16
    };
}

const KC_EDG_I: u16 = led!(EDG_I);
const KC_EDG_M: u16 = led!(EDG_M);
const KC_T_BR: u16 = led!(T_BR);
const KC_T_PTD: u16 = led!(T_PTD);
const KC_PSI: u16 = led!(PSI);
const KC_PSD: u16 = led!(PSD);
const KC_PTP: u16 = led!(PTP);
const KC_PTN: u16 = led!(PTN);
const KC_T_ONF: u16 = led!(T_ONF);
const KC_BRI: u16 = led!(BRI);
const KC_BRD: u16 = led!(BRD);
const KC_T_MD: u16 = led!(T_MD);

#[allow(non_camel_case_types)]
pub enum AltKeycodes {
    U_T_AGCR = LedKeycodes::T_PTD as isize + 1, //USB Toggle Automatic GCR control
    DBG_TOG,                                     //DEBUG Toggle On / Off
    DBG_MTRX,                                    //DEBUG Toggle Matrix Prints
    DBG_KBD,                                     //DEBUG Toggle Keyboard Prints
    DBG_MOU,                                     //DEBUG Toggle Mouse Prints
    DBG_FAC,                                     //DEBUG Factory light testing (All on white)
    MD_BOOT                                      //Restart into bootloader after hold timeout
}

macro_rules! cus {
    ( $x:ident ) => {
        AltKeycodes::$x as u16
    };
}

const KC_U_T_AGCR: u16 = cus!(U_T_AGCR);
const KC_MD_BOOT: u16 = cus!(MD_BOOT);
const KC_DBG_FAC: u16 = cus!(DBG_FAC);



const MATRIX_ROWS: usize = 5;
const MATRIX_COLS: usize = 15;

const LED_GCR_STEP: u8 = 10;
const LED_MODE_KEYS_ONLY: u8 = 1;
const LED_ANIMATION_GREEN: u8 = 5;
const LED_FLAG_USE_ROTATE_PATTERN: u16 = 0x40;

keymaps!(
    rows => MATRIX_ROWS,
    cols => MATRIX_COLS,
    layer_cnt => 4,
    layer!(  // layer 0
        r!(     '`'     | 1  | 2  | 3 | 4 | 5 |  6  | 7 | 8 | 9 |   0   |  -  | =  |BSPC |DEL  ),
        r!( [TAB &LT{3}]| Q  | W  | E | R | T |  Y  | U | I | O |   P   | '[' |']' |BSLS |HOME ),
        r!( [CAPS&LT{2}]| A  | S  | D | F | G |  H  | J | K | L |   ;   |QUOTE|xxxx|ENTER|PGUP ),
        r!(     LSFT    |xxxx| Z  | X | C | V |  B  | N | M | , |   .   |  /  |RSFT| '↑' |PGDN ),
        r!(     LCTL    |LALT|LGUI|xxx|xxx|xxx|SPACE|xxx|xxx|xxx|[MO{1}]|RGUI |'←' | '↓' | '→' ),
    ),
    layer!(  // layer 1
        r!( ESC  |F1 | F2 | F3  | F4  | F5  |  F6   |  F7   |   F8   | F9 |F10 |F11 |F12 |[  ]|'🔇' ),
        r!( T_BR |PSD|BRI | PSI |EDG_I|[   ]| [   ] | [   ] |U_T_AGCR|[  ]|PSCR|SLCK|PAUS|[  ]|END  ),
        r!( T_PTD|PTP|BRD | PTN |[   ]|[   ]|  '←'  |  '↓'  |   '↑'  |'→' |[  ]|[  ]|xxxx|[  ]|'🔊' ),
        r!( [  ] |xxx|T_MD|T_ONF|[   ]|EDG_M|MD_BOOT|TG_NKRO| [    ] |[  ]|[  ]|[  ]|[  ]|PGUP|'🔉' ),
        r!( [  ] |[ ]|[  ]|xxxxx|xxxxx|xxxxx|DBG_FAC|xxxxxxx|xxxxxxxx|xxxx|[  ]|[  ]|HOME|PGDN|END  ),
    ),
    layer!(  // layer 2
        r!([   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]),
        r!([   ]|  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  |  0  |  -  |  =  |[   ]|[   ]),
        r!([   ]|[   ]|[   ]|[   ]|[   ]|[   ]| '←' | '↓' | '↑' | '→' |[   ]|[   ]|xxxxx|[   ]|[   ]),
        r!([   ]|xxxxx|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]),
        r!([   ]|[   ]|[   ]|xxxxx|xxxxx|xxxxx|[   ]|xxxxx|xxxxx|xxxxx|[   ]|[   ]|[   ]|[   ]|[   ]),
    ),
    layer!(  // layer 3
        r!([   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]),
        r!([   ]|  !  |  @  |  #  |  $  |  %  |  ^  |  &  |  *  | '(' | ')' |  _  |  +  |[   ]|[   ]),
        r!([   ]|[   ]|[   ]|[   ]|[   ]|[   ]| '←' | '↓' | '↑' | '→' |[   ]|[   ]|xxxxx|[   ]|[   ]),
        r!([   ]|xxxxx|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]|[   ]),
        r!([   ]|[   ]|[   ]|xxxxx|xxxxx|xxxxx|[   ]|xxxxx|xxxxx|xxxxx|[   ]|[   ]|[   ]|[   ]|[   ]),
    ),
);

extern "C" {
    #[no_mangle]
    static mut led_animation_id: u8;
    static mut gcr_desired: u8;
    static mut led_lighting_mode: u8;
}

#[derive(Default)]
#[repr(C)]
pub struct led_instruction_t {
    pub flags: u16, // Bitfield for LED instructions
    pub id0: u32,   // Bitwise id, IDs 0-31
    pub id1: u32,   // Bitwise id, IDs 32-63
    pub id2: u32,   // Bitwise id, IDs 64-95
    pub id3: u32,   // Bitwise id, IDs 96-127
    pub layer: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub pattern_id: u8,
    pub end: u8,
}

/* weak attribute in tmk_core/protocol/arm_atsam/led_matrix.c */
#[no_mangle]
static led_instructions: [led_instruction_t; 2] = [
    //All LEDs use the user's selected pattern (this is the factory default)
    led_instruction_t{
        flags: LED_FLAG_USE_ROTATE_PATTERN,

        id0: 0,
        id1: 0,
        id2: 0,
        id3: 0,
        layer: 0,
        r: 0,
        g: 0,
        b: 0,
        pattern_id: 0,
        end: 0,
    },
    //end must be set to 1 to indicate end of instruction set
    led_instruction_t{
        end: 1,

        flags: 0,
        id0: 0,
        id1: 0,
        id2: 0,
        id3: 0,
        layer: 0,
        r: 0,
        g: 0,
        b: 0,
        pattern_id: 0,
    }
];

#[no_mangle]
pub extern "C" fn matrix_init_user() {
    unsafe {
        led_animation_id = LED_ANIMATION_GREEN;

        let gcr_decrease_step = LED_GCR_STEP * 12;
        if gcr_desired < gcr_decrease_step {
            gcr_desired = 0
        } else {
            gcr_desired -= gcr_decrease_step;
        }

        led_lighting_mode = LED_MODE_KEYS_ONLY;
    }
}

#[no_mangle]
pub extern "C" fn matrix_scan_user() {
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
