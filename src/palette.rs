use crate::themes::{Dark, Light};
use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) trait Palette {
    fn base(&self, scale: BaseScale) -> Oklch;
    fn keywords(&self) -> Oklch;
    fn strings(&self) -> Oklch;
    fn numbers(&self) -> Oklch;
    fn enum_members(&self) -> Oklch;
    fn function_declarations(&self) -> Oklch;
    fn types(&self) -> Oklch;
    fn enums(&self) -> Oklch;
    fn interfaces(&self) -> Oklch;
    fn type_aliases(&self) -> Oklch;
    fn builtin_types(&self) -> Oklch;
    fn namespace_declarations(&self) -> Oklch;
    fn constants(&self) -> Oklch;
    fn macros(&self) -> Oklch;
    fn attributes(&self) -> Oklch;
    fn try_operators(&self) -> Oklch;
}

impl Palette for Dark {
    fn base(&self, scale: BaseScale) -> Oklch {
        self.base(scale)
    }
    fn keywords(&self) -> Oklch {
        self.dark_purple()
    }
    fn strings(&self) -> Oklch {
        self.green()
    }
    fn numbers(&self) -> Oklch {
        self.green()
    }
    fn enum_members(&self) -> Oklch {
        self.red()
    }
    fn function_declarations(&self) -> Oklch {
        self.dark_green()
    }
    fn types(&self) -> Oklch {
        self.turquoise()
    }
    fn enums(&self) -> Oklch {
        self.light_green()
    }
    fn interfaces(&self) -> Oklch {
        self.purple()
    }
    fn type_aliases(&self) -> Oklch {
        self.orange()
    }
    fn builtin_types(&self) -> Oklch {
        self.blue()
    }
    fn namespace_declarations(&self) -> Oklch {
        self.yellow()
    }
    fn constants(&self) -> Oklch {
        self.light_blue()
    }
    fn macros(&self) -> Oklch {
        self.teal()
    }
    fn attributes(&self) -> Oklch {
        self.red()
    }
    fn try_operators(&self) -> Oklch {
        self.orange2()
    }
}

impl Palette for Light {
    fn base(&self, scale: BaseScale) -> Oklch {
        self.base(scale)
    }
    fn keywords(&self) -> Oklch {
        self.purple()
    }
    fn strings(&self) -> Oklch {
        self.olive()
    }
    fn numbers(&self) -> Oklch {
        self.olive()
    }
    fn enum_members(&self) -> Oklch {
        self.red()
    }
    fn function_declarations(&self) -> Oklch {
        self.brown()
    }
    fn types(&self) -> Oklch {
        self.pink()
    }
    fn enums(&self) -> Oklch {
        self.green()
    }
    fn interfaces(&self) -> Oklch {
        self.deep_purple()
    }
    fn type_aliases(&self) -> Oklch {
        self.orange()
    }
    fn builtin_types(&self) -> Oklch {
        self.teal()
    }
    fn namespace_declarations(&self) -> Oklch {
        self.blue()
    }
    fn constants(&self) -> Oklch {
        self.dark_blue()
    }
    fn macros(&self) -> Oklch {
        self.cyan()
    }
    fn attributes(&self) -> Oklch {
        self.red()
    }
    fn try_operators(&self) -> Oklch {
        self.orange2()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    Fg,
}

pub(crate) fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

pub(crate) fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
