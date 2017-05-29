struct Foo<'a> {  // 'a is a generic indicating that we have a lifetime named a
    x: &'a i32, // x is a reference with lifetime 'a
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    let y = &5;
    let f = Foo { x: y }; // compiler validate that f lifetime is >= than lifetime of reference given for x
	
    println!("x is: {}", f.x());
}
