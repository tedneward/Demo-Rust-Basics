// {{## BEGIN comments ##}}
// This is a standard comment

/// This is a documentation comment.
/// It supports Markdown syntax.
/// 
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
// {{## END comments ##}}

// {{## BEGIN basics ##}}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
// {{## END basics ##}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
