use crate::palette::{lerp, oklch, BaseScale};
use tincture::Oklch;

pub(crate) struct Light;

impl Light {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        let value = match scale {
            BaseScale::Bg => 0.0,
            BaseScale::Fg => 1.0,
        };

        let lightness = lerp(value, 1.0..0.0);

        oklch(lightness, 0.0, 0.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(0.5, 0.2, 25.0)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.6, 0.14, 55.0)
    }

    pub(crate) fn brown(&self) -> Oklch {
        oklch(0.6, 0.1, 75.0)
    }

    pub(crate) fn olive(&self) -> Oklch {
        oklch(0.6, 0.14, 120.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.6, 0.08, 150.0)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.6, 0.09, 200.0)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.6, 0.08, 215.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.6, 0.1, 250.0)
    }

    pub(crate) fn dark_blue(&self) -> Oklch {
        oklch(0.5, 0.05, 250.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.5, 0.13, 300.0)
    }

    pub(crate) fn deep_purple(&self) -> Oklch {
        oklch(0.6, 0.2, 300.0)
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(0.6, 0.16, 340.0)
    }
}
