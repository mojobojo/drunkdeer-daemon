// NOTE: I mapped this from the G60, possibly not the same for all keyboards and the full size keyboards have more keys than I have mapped here.
// NOTE: Subtract 128 and you get the ID value they use in the web ui.

#[allow(dead_code)]
pub mod key_id {
    // Row 1 (top)
    pub const ESCAPE: u8 = 0x95;
    pub const KEY_1: u8 = 0x96;
    pub const KEY_2: u8 = 0x97;
    pub const KEY_3: u8 = 0x98;
    pub const KEY_4: u8 = 0x99;
    pub const KEY_5: u8 = 0x9A;
    pub const KEY_6: u8 = 0x9B;
    pub const KEY_7: u8 = 0x9C;
    pub const KEY_8: u8 = 0x9D;
    pub const KEY_9: u8 = 0x9E;
    pub const KEY_0: u8 = 0x9F;
    pub const MINUS: u8 = 0xA0;
    pub const EQUALS: u8 = 0xA1;
    pub const BACKSPACE: u8 = 0xA2;

    // Row 2
    pub const TAB: u8 = 0xAA;
    pub const Q: u8 = 0xAB;
    pub const W: u8 = 0xAC;
    pub const E: u8 = 0xAD;
    pub const R: u8 = 0xAE;
    pub const T: u8 = 0xAF;
    pub const Y: u8 = 0xB0;
    pub const U: u8 = 0xB1;
    pub const I: u8 = 0xB2;
    pub const O: u8 = 0xB3;
    pub const P: u8 = 0xB4;
    pub const LBRACKET: u8 = 0xB5;
    pub const RBRACKET: u8 = 0xB6;
    pub const BACKSLASH: u8 = 0xB7;

    // Row 3
    pub const CAPSLOCK: u8 = 0xBF;
    pub const A: u8 = 0xC0;
    pub const S: u8 = 0xC1;
    pub const D: u8 = 0xC2;
    pub const F: u8 = 0xC3;
    pub const G: u8 = 0xC4;
    pub const H: u8 = 0xC5;
    pub const J: u8 = 0xC6;
    pub const K: u8 = 0xC7;
    pub const L: u8 = 0xC8;
    pub const SEMICOLON: u8 = 0xC9;
    pub const APOSTROPHE: u8 = 0xCA;
    pub const ENTER: u8 = 0xCC;

    // Row 4
    pub const LSHIFT: u8 = 0xD4;
    pub const Z: u8 = 0xD6;
    pub const X: u8 = 0xD7;
    pub const C: u8 = 0xD8;
    pub const V: u8 = 0xD9;
    pub const B: u8 = 0xDA;
    pub const N: u8 = 0xDB;
    pub const M: u8 = 0xDC;
    pub const COMMA: u8 = 0xDD;
    pub const PERIOD: u8 = 0xDE;
    pub const FORWARDSLASH: u8 = 0xDF;
    pub const RSHIFT: u8 = 0xE1;

    // Row 5 (bottom)
    pub const LCTRL: u8 = 0xE9;
    pub const LWIN: u8 = 0xEA;
    pub const LALT: u8 = 0xEB;
    pub const SPACE: u8 = 0xEF;
    pub const RALT: u8 = 0xF3;
    pub const FN: u8 = 0xF4;
    pub const MENU: u8 = 0xF5;
    pub const RCTRL: u8 = 0xF6;
}
