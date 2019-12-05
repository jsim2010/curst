use curst::Key;

#[test]
fn from_cint() {
    let cases = vec![
        (-2, Key::Unknown(-2)),
        (0x08, Key::Backspace),
        (0x09, Key::Tab),
        (0x0A, Key::Enter),
        (0x1B, Key::Esc),
        (0x20, Key::Printable(' ')),
        (0x30, Key::Printable('0')),
        (0x41, Key::Printable('A')),
        (0x61, Key::Printable('a')),
    ];

    for case in cases {
        assert_eq!(Key::from(case.0), case.1);
    }
}
