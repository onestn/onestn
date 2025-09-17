fn len_ref(s: &String) -> usize { s.len() }

fn push_ex(s: &mut String) { s.push_str(" world"); }

fn main() {
    let mut s = String::from("hello");
    let l = len_ref(&s); // 불변 빌림
    println!("{l}");

    push_ex(&mut s);     // 가변 빌림
    println!("{s}");
}
