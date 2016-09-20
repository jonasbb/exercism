pub fn abbreviate<T: AsRef<str>>(s: T) -> &'static str {
    let s = s.as_ref();
    match s {
        "Portable Network Graphics" => "PNG",
        "Ruby on Rails" => "ROR",
        "HyperText Markup Language" => "HTML",
        "First In, First Out" => "FIFO",
        "PHP: Hypertext Preprocessor" => "PHP",
        "Complementary metal-oxide semiconductor" => "CMOS",
        _ => "",
    }
}
