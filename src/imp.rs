use crate::palette::{BaseScale, Palette};
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &impl Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &impl Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["editor.foreground", "foreground"],
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::FadedFg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &impl Palette) {
    builder.add_rule(Semantic("keyword"), palette.keywords());

    builder.add_rules(
        &[
            Semantic("operator"),
            Semantic("arithmetic"),
            Semantic("comparison"),
            Semantic("logical"),
            Semantic("bitwise"),
        ],
        palette.operators(),
    );

    builder.add_rules(
        &[
            Semantic("string"),
            Semantic("character"),
            Semantic("number"),
        ],
        palette.literals(),
    );

    builder.add_rules(
        &[Semantic("boolean"), Semantic("enumMember")],
        palette.enum_members(),
    );

    builder.add_rules(
        &[
            Semantic("function.declaration"),
            Semantic("method.declaration"),
        ],
        palette.function_declarations(),
    );

    builder.add_rule(Semantic("struct"), palette.types());
    builder.add_rule(Semantic("enum"), palette.enums());
    builder.add_rule(Semantic("interface"), palette.interfaces());
    builder.add_rule(Semantic("typeAlias"), palette.type_aliases());
    builder.add_rule(Semantic("builtinType"), palette.builtin_types());
    builder.add_rule(
        Semantic("namespace.declaration"),
        palette.namespace_declarations(),
    );
    builder.add_rules(
        &[Semantic("variable.constant"), Semantic("variable.static")],
        palette.constants(),
    );

    builder.add_rule(Semantic("lifetime"), palette.lifetimes());

    builder.add_rule(Semantic("macro"), palette.macros());
    builder.add_rule(Semantic("*.attribute"), palette.attributes());

    builder.add_rule(Semantic("comment"), palette.base(BaseScale::FadedFg));
}
