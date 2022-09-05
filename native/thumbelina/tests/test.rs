use rustler::nif;

#[cfg(test)]
mod tests {

    #[test]
    fn echo_test() {
        assert_eq!(echo("hello"), "hello")
    }
}
