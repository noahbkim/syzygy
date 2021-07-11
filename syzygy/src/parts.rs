#[derive(Debug, Eq, PartialEq)]
pub enum Parts<'a> {
    Cons(&'a str, &'a str),
    Nil,
}

impl<'a> From<&'a str> for Parts<'a> {
    fn from(mut path: &'a str) -> Self {
        path = path.trim_start_matches('/');
        if path.len() > 0 {
            let slash = path.find('/').unwrap_or(path.len());
            Parts::from(path.split_at(slash))
        } else {
            Parts::Nil
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for Parts<'a> {
    fn from(parts: (&'a str, &'a str)) -> Self {
        Parts::Cons(parts.0, parts.1)
    }
}

#[cfg(test)]
mod tests {
    use super::Parts;

    #[test]
    fn one_part() {
        assert_eq!(Parts::from("/a"), Parts::Cons("a", ""));
    }

    #[test]
    fn two_parts() {
        assert_eq!(Parts::from("/a/b"), Parts::Cons("a", "/b"));
    }

    #[test]
    fn one_part_tail() {
        assert_eq!(Parts::from("/a/"), Parts::Cons("a", "/"));
    }

    #[test]
    fn empty() {
        assert_eq!(Parts::from(""), Parts::Nil);
    }

    #[test]
    fn one_slash() {
        assert_eq!(Parts::from("/"), Parts::Nil);
    }

    #[test]
    fn two_slashes() {
        assert_eq!(Parts::from("//"), Parts::Nil);
    }
}
