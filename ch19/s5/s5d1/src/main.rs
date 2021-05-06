type StrRef<'a> = &'a str;

fn print_str<'b>(s: StrRef<'b>) {
    println!("{}", s);
}

fn main() {
   
    let s: StrRef<'static> = "hello";
    print_str(s);

}
