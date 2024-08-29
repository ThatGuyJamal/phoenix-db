pub fn log(msg: &str)
{
    println!("{}", msg);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn it_works()
    {
        log("Hello, world!");

        assert_eq!(2 + 2, 4);
    }
}
