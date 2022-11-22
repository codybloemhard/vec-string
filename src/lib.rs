pub trait VecString{
    fn vec_string(&self) -> String;
    fn vec_string_inner(&self) -> String;
}

/// Get string of Vec<T> where T: Display
impl<T> VecString for Vec<T> where T: std::fmt::Display{
    /// assert_eq!("[1, 2, 3]", vec![1, 2, 3].vec_string());
    fn vec_string(&self) -> String{
        format!("[{}]", self.vec_string_inner())
    }

    /// assert_eq!("1, 2, 3", vec![1, 2, 3].vec_string());
    fn vec_string_inner(&self) -> String{
        let mut string = String::new();
        for x in self{
            string.push_str(&format!("{}, ", x));
        }
        string.pop();
        string.pop();
        string
    }
}

#[cfg(test)]
mod tests {
    use crate::VecString;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test() {
        assert_eq!("[1, 2, 3]", vec![1, 2, 3].vec_string());
        assert_eq!("1, 2, 3", vec![1, 2, 3].vec_string_inner());
    }
}
