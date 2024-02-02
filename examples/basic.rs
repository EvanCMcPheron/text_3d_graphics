use text_3d_graphics::printing::CharBuffer;

fn main() {
    let cb = CharBuffer::from_str("   \n @ \n @@").unwrap();
    println!("{cb}");
}
