pub mod printer;
#[cfg(test)]
mod tests {
    #[test]
    fn hello() {
        use std::io::{stdout, Write};
        let l = stdout();
        let l = l.lock();
        let mut w = super::printer::Printer::new(l);
        assert_eq!(w.write("HELLO".as_bytes()).unwrap(), 5);
    }
}
