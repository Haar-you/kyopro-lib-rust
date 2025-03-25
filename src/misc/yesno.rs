pub trait YesNo {
    fn yesno<'a>(self, y: &'a str, n: &'a str) -> &'a str;
}

impl YesNo for bool {
    fn yesno<'a>(self, y: &'a str, n: &'a str) -> &'a str {
        if self {
            y
        } else {
            n
        }
    }
}
