mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let palette = palette::Palette;

    let mut theme = ThemeBuilder::new("rustdoc dark".to_string(), Type::Dark);
    imp::add_rules(&mut theme, &palette);
    theme.build().save()?;

    Ok(())
}
