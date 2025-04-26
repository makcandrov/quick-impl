use syn::LitStr;

pub fn to_snake_case(variant: &str) -> String {
    let mut snake = String::new();
    for (i, ch) in variant.char_indices() {
        if i > 0 && ch.is_uppercase() {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    snake
}

pub fn runtime_format(
    base: impl AsRef<str>,
    args: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    let mut res = base.as_ref().to_string();
    for (i, arg) in args.into_iter().enumerate() {
        res = res.replacen("{}", arg.as_ref(), 1);
        res = res.replace(&format!("{{{i}}}"), arg.as_ref());
    }
    res
}

pub fn set_lit_str_value(lit_str: &mut LitStr, new_value: impl AsRef<str>) {
    *lit_str = LitStr::new(new_value.as_ref(), lit_str.span());
}

pub trait TryRetain<T> {
    fn try_retain<F, E>(&mut self, mut f: F) -> Result<(), E>
    where
        F: FnMut(&T) -> Result<bool, E>,
    {
        self.try_retain_mut(|elem| f(elem))
    }

    fn try_retain_mut<F, E>(&mut self, f: F) -> Result<(), E>
    where
        F: FnMut(&mut T) -> Result<bool, E>;
}

impl<T> TryRetain<T> for Vec<T> {
    fn try_retain_mut<F, E>(&mut self, mut f: F) -> Result<(), E>
    where
        F: FnMut(&mut T) -> Result<bool, E>,
    {
        let mut err = None::<E>;

        self.retain_mut(|elem| {
            if err.is_some() {
                return true;
            }
            match f(elem) {
                Ok(r) => r,
                Err(e) => {
                    err.replace(e);
                    true
                }
            }
        });

        err.map_or(Ok(()), Err)
    }
}

pub trait ThenTry {
    fn then_try<T, E, F: FnOnce() -> Result<T, E>>(self, f: F) -> Result<Option<T>, E>;
}

impl ThenTry for bool {
    fn then_try<T, E, F: FnOnce() -> Result<T, E>>(self, f: F) -> Result<Option<T>, E> {
        if self { f().map(Some) } else { Ok(None) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_format() {
        assert_eq!(
            &runtime_format("test{}test{0}{}test{1}{0}", ["A", "B"]),
            "testAtestABtestBA",
        )
    }
}
