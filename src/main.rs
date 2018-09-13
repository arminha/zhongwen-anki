#[macro_use]
extern crate lazy_static;
extern crate unicode_segmentation;

mod pinyin;

fn main() {
    println!("你好！");
    println!("{}", pinyin::numbers_to_marks("Ni3hao3!"));
}
