mod imp;
mod palette;
mod themes;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    gen_theme("rustdoc ayu", themes::Ayu, Type::Dark)?;
    gen_theme("rustdoc dark", themes::Dark, Type::Dark)?;
    gen_theme("rustdoc light", themes::Light, Type::Light)?;

    Ok(())
}

fn gen_theme(name: &str, palette: impl palette::Palette, ty: Type) -> io::Result<()> {
    let mut builder = ThemeBuilder::new(name.to_string(), ty);
    imp::add_rules(&mut builder, &palette);
    builder.build().save()?;

    Ok(())
}
