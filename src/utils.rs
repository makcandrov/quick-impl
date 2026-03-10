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
