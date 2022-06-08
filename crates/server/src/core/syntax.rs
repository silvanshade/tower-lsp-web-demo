use crate::core::session::Session;

pub(crate) fn update_channel(tree: Option<&tree_sitter::Tree>) {
    // assume errors; use red
    let mut color = "rgb(255, 87, 51)";
    if let Ok(channel_syntax) = Session::get_channel_syntax() {
        if let Some(tree) = tree {
            let sexp = crate::format_sexp(tree.root_node().to_sexp());
            channel_syntax.set_value(&sexp);
            if !tree.root_node().has_error() {
                // no errors; use green
                color = "rgb(218, 247, 166)";
            }
        }
        channel_syntax
            .style()
            .set_property("background-color", color)
            .expect("failed to set style");
    }
}
