use std::libc::c_int;

pub type BlendMode = c_int;

pub static BlendModeNone  : BlendMode = 0;
pub static BlendModeBlend : BlendMode = 1;
pub static BlendModeAdd   : BlendMode = 2;
pub static BlendModeMod   : BlendMode = 4;
