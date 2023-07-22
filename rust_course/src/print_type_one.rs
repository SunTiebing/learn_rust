pub fn print() {
    let ch: u32 = 'Z' as u32;
    for ch in ((char::from_u32(ch + 1).unwrap())..'a').rev() {
        println!("{}", ch);
    }
}
