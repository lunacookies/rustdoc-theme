use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["editor.foreground", "foreground"],
        palette.base(BaseScale::Fg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("keyword"), palette.dark_purple());

    builder.add_rules(
        &[Semantic("string"), Semantic("character")],
        palette.green(),
    );

    builder.add_rule(Semantic("number"), palette.green());

    builder.add_rules(
        &[Semantic("boolean"), Semantic("enumMember")],
        palette.red(),
    );

    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
        ],
        palette.dark_green(),
    );

    builder.add_rule(Semantic("struct"), palette.turquoise());
    builder.add_rule(Semantic("enum"), palette.light_green());
    builder.add_rule(Semantic("interface"), palette.purple());
    builder.add_rule(Semantic("typeAlias"), palette.orange());
    builder.add_rule(Semantic("builtinType"), palette.blue());
    builder.add_rule(Semantic("namespace.declaration"), palette.yellow());

    builder.add_rule(Semantic("macro"), palette.teal());
    builder.add_rule(Semantic("*.attribute"), palette.red());

    builder.add_rule(
        Semantic("operator.controlFlow"),
        (palette.orange2(), FontStyle::Bold),
    );
}
