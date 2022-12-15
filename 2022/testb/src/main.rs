fn main() {
    let mystring = "123456";
    let splitter = mystring.split_at(mystring.len() / 2);
    println!("{}\t{}", splitter.0, splitter.1);
}
