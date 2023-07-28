pub struct Arguments<'a> {
    pub filename: &'a str,
}

impl<'a> Arguments<'a> {
    pub fn new(args: &'a [String]) -> Arguments<'a> {
        Arguments { filename: &args[1] }
    }
}
