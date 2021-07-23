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
        oklch(0.54233915, 0.19544585, 26.500795)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.5797153, 0.14561616, 53.788174)
    }

    pub(crate) fn brown(&self) -> Oklch {
        oklch(0.5712687, 0.09536953, 72.57695)
    }

    pub(crate) fn olive(&self) -> Oklch {
        oklch(0.5973572, 0.14521047, 122.22972)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.55647177, 0.08375478, 147.87569)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.63131934, 0.085633695, 201.28671)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.5580852, 0.0835976, 215.83727)
    }

    pub(crate) fn dark_blue(&self) -> Oklch {
        oklch(0.52880263, 0.053979643, 250.7377)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.5606956, 0.09898613, 256.47968)
    }

    pub(crate) fn deep_purple(&self) -> Oklch {
        oklch(0.58687973, 0.21805903, 287.97256)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.5489858, 0.12862441, 310.80942)
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(0.5521267, 0.16140372, 340.807)
    }
}
