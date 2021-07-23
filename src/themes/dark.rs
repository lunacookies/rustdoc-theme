use crate::palette::{lerp, oklch, BaseScale};
use tincture::Oklch;

pub(crate) struct Dark;

impl Dark {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        let value = match scale {
            BaseScale::Bg => 0.0,
            BaseScale::Fg => 1.0,
        };

        let lightness = lerp(value, 0.33..0.9);

        oklch(lightness, 0.0, 0.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(0.6824157, 0.16623303, 22.491032)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.7304144, 0.18637057, 52.56002)
    }

    pub(crate) fn orange2(&self) -> Oklch {
        oklch(0.7573304, 0.17531186, 59.37576)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.7104691, 0.14609218, 96.012085)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.667013, 0.16276555, 122.56789)
    }

    pub(crate) fn light_green(&self) -> Oklch {
        oklch(0.7134207, 0.07480228, 148.8825)
    }

    pub(crate) fn dark_green(&self) -> Oklch {
        oklch(0.65620345, 0.15235366, 153.44058)
    }

    pub(crate) fn turquoise(&self) -> Oklch {
        oklch(0.7304256, 0.117167644, 189.98401)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.63131934, 0.085633695, 201.28671)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.69966334, 0.102494046, 216.1858)
    }

    pub(crate) fn light_blue(&self) -> Oklch {
        oklch(0.70906216, 0.06555244, 249.50156)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.72096276, 0.14965181, 302.22723)
    }

    pub(crate) fn dark_purple(&self) -> Oklch {
        oklch(0.6834442, 0.087084174, 311.37656)
    }
}
