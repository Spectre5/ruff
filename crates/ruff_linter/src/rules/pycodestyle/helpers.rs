/// Returns `true` if the name should be considered "ambiguous".
pub(super) fn is_ambiguous_name(name: &str) -> bool {
    name == "l" || name == "I" || name == "O"
}
