//! CSS display ownership shared by the web renderer and host regression tests.
use std::collections::HashMap;

/// Base rendering owns display, including visibility. List rendering owns only
/// direction/alignment/gap: its change ticks need not coincide with base rendering.
pub(crate) fn apply_display(
    styles: &mut HashMap<String, String>,
    visible: bool,
    linear_list: bool,
    label: bool,
) {
    let display = match (visible, linear_list, label) {
        (false, _, _) => "none",
        (true, true, _) => "flex",
        (true, false, true) => "block",
        _ => "grid",
    };
    styles.insert("display".into(), display.into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_only_refresh_preserves_horizontal_button_layout() {
        let mut css = HashMap::from([
            ("flex-direction".into(), "row".into()),
            ("width".into(), "100%".into()),
        ]);
        // Visibility updates, then later content/hover/measurement updates.
        // The list renderer may not run during any of these base-only passes.
        for visible in [true, false, true, true, true] {
            apply_display(&mut css, visible, true, false);
            assert_eq!(css["display"], if visible { "flex" } else { "none" });
            assert_eq!(css["flex-direction"], "row");
            assert_eq!(css["width"], "100%");
        }
    }

    #[test]
    fn vertical_editor_stays_flex_after_input_changes() {
        let mut css = HashMap::from([
            ("flex-direction".into(), "column".into()),
            ("width".into(), "100%".into()),
        ]);
        apply_display(&mut css, false, true, false);
        apply_display(&mut css, true, true, false);
        // A later InputField/Name/Transform change must not turn the panel into a grid.
        apply_display(&mut css, true, true, false);
        assert_eq!(css["display"], "flex");
        assert_eq!(css["flex-direction"], "column");
        assert_eq!(css["width"], "100%");
    }

    #[test]
    fn ordinary_controls_labels_and_hidden_lists_keep_their_display_modes() {
        let mut css = HashMap::new();
        for (visible, list, label, expected) in [
            (true, false, false, "grid"),
            (true, false, true, "block"),
            (true, true, true, "flex"),
            (false, true, false, "none"),
        ] {
            apply_display(&mut css, visible, list, label);
            assert_eq!(css["display"], expected);
        }
    }
}
