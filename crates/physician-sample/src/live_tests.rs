use super::live;

#[test]
fn chat_titles_are_one_line_and_cut_at_a_word() {
    assert_eq!(live::chat_title("  About\n the   labs "), "About the labs");
    let title = live::chat_title(
        "Please review the potassium trend and tell me whether the latest result changes the plan",
    );
    assert_eq!(
        title,
        "Please review the potassium trend and tell me whether the…"
    );
    assert!(title.chars().count() <= live::CHAT_TITLE_CHARS + 1);
    let unbroken = "x".repeat(80);
    assert_eq!(
        live::chat_title(&unbroken).chars().count(),
        live::CHAT_TITLE_CHARS + 1
    );
}
