fn main() {
    let path = std::env::args().nth(1).expect("'path' must be provided.");
    let out = pdf_extract::extract_text(path).unwrap();
    println!("{out}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let out = pdf_extract::extract_text("dummy.pdf").unwrap();
        assert_eq!("\n\nDummy PDF file", out);
    }
}
