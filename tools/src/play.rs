pub fn bool_iter(pred: bool) -> impl Iterator<Item = ()> {
    if_some(pred, ()).into_iter()
}

pub fn bool_iter_val<T>(pred: bool, val: T) -> impl Iterator<Item = T> {
    if_some(pred, val).into_iter()
}

pub fn bool_iter_with<T>(pred: bool, f: impl FnOnce() -> T) -> impl Iterator<Item = T> {
    if_some_with(pred, f).into_iter()
}

pub fn if_some<T>(pred: bool, val: T) -> Option<T> {
    if pred {
        Some(val)
    } else {
        None
    }
}

pub fn if_some_with<T>(pred: bool, f: impl FnOnce() -> T) -> Option<T> {
    if pred {
        Some(f())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_if_some() {
        assert_eq!(Some(3), if_some(true, 3));
        assert_eq!(None, if_some(false, 3));
    }

    #[test]
    fn test_if_some_with() {
        assert_eq!(Some(8), if_some_with(true, || 8));
        assert_eq!(None, if_some_with(false, || 8));
    }

    #[test]
    fn test_bool_iter() {
        let mut i = bool_iter(true);
        assert_eq!(Some(()), i.next());
        assert_eq!(None, i.next());
        assert_eq!(None, i.next());

        let mut i = bool_iter(false);
        assert_eq!(None, i.next());
        assert_eq!(None, i.next());
    }

    #[test]
    fn test_bool_iter_val() {
        let mut i = bool_iter_val(true, 81);
        assert_eq!(Some(81), i.next());
        assert_eq!(None, i.next());
        assert_eq!(None, i.next());

        let mut i = bool_iter_val(false, 66);
        assert_eq!(None, i.next());
        assert_eq!(None, i.next());
    }

    #[test]
    fn test_bool_iter_with() {
        let mut i = bool_iter_with(true, || 81);
        assert_eq!(Some(81), i.next());
        assert_eq!(None, i.next());
        assert_eq!(None, i.next());

        let mut i = bool_iter_with(false, || 66);
        assert_eq!(None, i.next());
        assert_eq!(None, i.next());
    }
}
