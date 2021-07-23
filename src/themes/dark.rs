use crate::palette::{lerp, oklch, BaseScale};
use tincture::Oklch;

pub(crate) struct Dark;

impl Dark {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        let value = match scale {
            BaseScale::Bg => 0.0,
            BaseScale::FadedFg => 0.5,
            BaseScale::Fg => 1.0,
        };

        let lightness = lerp(value, 0.33..0.9);

        oklch(lightness, 0.0, 0.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(0.7, 0.15, 20.0)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.7, 0.17, 55.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.7, 0.14, 95.0)
    }

    pub(crate) fn olive(&self) -> Oklch {
        oklch(0.7, 0.16, 125.0)
    }

    pub(crate) fn sea_green(&self) -> Oklch {
        oklch(0.7, 0.07, 150.0)
    }

    pub(crate) fn felt(&self) -> Oklch {
        oklch(0.65, 0.15, 155.0)
    }

    pub(crate) fn turquoise(&self) -> Oklch {
        oklch(0.7, 0.12, 190.0)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.6, 0.09, 200.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.7, 0.1, 215.0)
    }

    pub(crate) fn indigo(&self) -> Oklch {
        oklch(0.7, 0.07, 250.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.7, 0.09, 300.0)
    }

    pub(crate) fn deep_purple(&self) -> Oklch {
        oklch(0.7, 0.15, 300.0)
    }
}
