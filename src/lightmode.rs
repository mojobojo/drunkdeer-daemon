#[allow(dead_code)]
pub mod light_mode {

    pub const OFF: u8 = 0;
    pub const RAINBOWMARQUEE: u8 = 1;
    pub const WAVETTSPECTRUM: u8 = 2;
    pub const SURFRIGHT: u8 = 3;
    pub const BREATH: u8 = 4;
    pub const CENTERSURFING: u8 = 5;
    pub const SPECTRUM: u8 = 6;
    pub const RIPPLE: u8 = 7;
    pub const ALWAYSLIGHT: u8 = 8;
    pub const LIGHTBYPRESS: u8 = 9;
    pub const SERPENT: u8 = 10;
    pub const COLORFULFOUNTAIN: u8 = 11;
    pub const LASERKEY: u8 = 12;
    pub const GLOWINGFAN: u8 = 13;
    pub const SURFINGCROSS: u8 = 14;
    pub const HEART: u8 = 15;
    pub const TRAFFIC: u8 = 16;
    pub const GSTRIKE: u8 = 17;
    pub const RAINDROPS: u8 = 18;
    pub const STATIC: u8 = 19;
    pub const UNKNOWN: u8 = 20; // demo/test mode?
}

#[allow(dead_code)]
pub mod light_color {
    pub const RAINBOW: u8 = 0;
    pub const RED: u8 = 1;
    pub const GREEN: u8 = 2;
    pub const BLUE: u8 = 3;
    pub const YELLOW: u8 = 4;
    pub const MAGENTA: u8 = 5;
    pub const CYAN: u8 = 6;
    pub const WHITE: u8 = 7;

    // NOTE: this is for when you are setting a custom color to specific keys.
    pub const SETKEY: u8 = 19;
}
