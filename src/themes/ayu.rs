use crate::palette::{lerp, oklch, BaseScale};
use tincture::Oklch;

pub(crate) struct Ayu;

impl Ayu {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        let value = match scale {
            BaseScale::Bg => 0.0,
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
        oklch(0.80276597, 0.11327749, 16.468979)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.72073126, 0.18333054, 44.073963)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.89242685, 0.10684289, 84.08479)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.8061073, 0.14816323, 117.721115)
    }

    pub(crate) fn turquoise(&self) -> Oklch {
        oklch(0.853207, 0.07823637, 172.16351)
    }

    pub(crate) fn dark_blue(&self) -> Oklch {
        oklch(0.5834534, 0.085390955, 215.18198)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.70612586, 0.1169701, 225.16397)
    }

    pub(crate) fn dark_indigo(&self) -> Oklch {
        oklch(0.5898327, 0.059607137, 251.32379)
    }

    pub(crate) fn indigo(&self) -> Oklch {
        oklch(0.837296, 0.07228205, 256.94122)
    }

    pub(crate) fn lilac(&self) -> Oklch {
        oklch(0.830168, 0.081171565, 299.87268)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.6516097, 0.12597175, 306.204)
    }
}
