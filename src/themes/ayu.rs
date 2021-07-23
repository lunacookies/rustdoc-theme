use crate::palette::{lerp, oklch, BaseScale};
use tincture::Oklch;

pub(crate) struct Ayu;

impl Ayu {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        let value = match scale {
            BaseScale::Bg => 0.0,
            BaseScale::FadedFg => 0.6,
            BaseScale::Fg => 1.0,
        };

        let lightness = lerp(value, 0.19..0.9);

        let chroma = lerp(value, 0.01..0.02);

        let light_hue = 95.0;
        let dark_hue = 250.0;

        let hue = if value > 0.9 { light_hue } else { dark_hue };

        oklch(lightness, chroma, hue)
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(0.8, 0.11, 15.0)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.7, 0.18, 45.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.9, 0.1, 85.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.8, 0.15, 120.0)
    }

    pub(crate) fn turquoise(&self) -> Oklch {
        oklch(0.85, 0.08, 170.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.7, 0.12, 225.0)
    }

    pub(crate) fn dark_blue(&self) -> Oklch {
        oklch(0.6, 0.09, 225.0)
    }

    pub(crate) fn indigo(&self) -> Oklch {
        oklch(0.85, 0.07, 255.0)
    }

    pub(crate) fn dark_indigo(&self) -> Oklch {
        oklch(0.6, 0.06, 255.0)
    }

    pub(crate) fn lilac(&self) -> Oklch {
        oklch(0.85, 0.08, 300.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.65, 0.13, 305.0)
    }
}
