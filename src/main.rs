use char_type::get_char_type;

pub mod char_type;

fn main() {
    let a = get_char_type(' ');
    println!("Hello, world! {:?}", a);
}
