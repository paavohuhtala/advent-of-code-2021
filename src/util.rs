pub fn parse_lines(str: &str) -> Vec<i32> {
    str.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

pub trait BoolIterUtil {
    fn all_true(&mut self) -> bool;
    fn any_true(&mut self) -> bool;
}

// What is the standard library equivalent of this?
pub trait AsOwned<T> {
    fn as_owned(self) -> T;
}

impl<T> AsOwned<T> for T {
    fn as_owned(self) -> T {
        self
    }
}

impl AsOwned<bool> for &bool {
    fn as_owned(self) -> bool {
        *self
    }
}

impl<I, B: AsOwned<bool>> BoolIterUtil for I
where
    I: Iterator<Item = B>,
{
    fn all_true(&mut self) -> bool {
        self.all(|b| b.as_owned())
    }

    fn any_true(&mut self) -> bool {
        self.any(|b| b.as_owned())
    }
}
