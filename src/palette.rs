use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 265.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(0.6824157, 0.16623303, 22.491032)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.667013, 0.16276555, 122.56789)
    }

    pub(crate) fn dark_green(&self) -> Oklch {
        oklch(0.65620345, 0.15235366, 153.44058)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.63131934, 0.085633695, 201.28671)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.72096276, 0.14965181, 302.22723)
    }

    pub(crate) fn dark_purple(&self) -> Oklch {
        oklch(0.6834442, 0.087084174, 311.37656)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    Fg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.33..0.9)
    }

    fn chroma(self) -> f32 {
        0.0
    }

    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::Fg => 1.0,
        }
    }
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
