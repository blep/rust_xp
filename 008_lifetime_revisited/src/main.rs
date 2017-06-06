trait Alive {
    fn alive(&self);
}


// Obj: simple struct
// ////////////////////////////////////////////////////////

struct Obj {
    name: String,
}

impl Obj {
    fn new( name: &str ) -> Obj {
        let o = Obj{ name: name.to_string() };
        println!("+ Constructed Obj name={}", name);
        o
    }
}

impl Alive for Obj {
    fn alive(&self) {
        println!("= Obj name={} is alive", self.name);
    }
}

impl Drop for Obj {
    fn drop(&mut self) {
        println!("- Dropping Obj name={}", self.name);
    }
}


// Obj: copyable struct
// ////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct CopyObj {
    name: &'static str,
}


// testing
// ////////////////////////////////////////////////////////


fn print_alive_by_ref<T: Alive>(o: &T) {
    println!("> enter print_alive_by_ref()");
    o.alive();
    println!("< exit print_alive_by_ref()");
}

fn print_alive_by_mut_ref<T: Alive>(o: &mut T) {
    println!("> enter print_alive_by_mut_ref()");
    o.alive();
    println!("< exit print_alive_by_mut_ref()");
}

fn print_alive_by_value<T: Alive>(o: T) {
    println!("> enter print_alive_by_value()");
    o.alive();
    println!("< exit print_alive_by_value()");
}



fn main() {
    println!("> enter main()");
    let mut o1 = Obj::new( "main" );
    o1.alive();
    print_alive_by_ref(&o1);
    print_alive_by_mut_ref(&mut o1);
    print_alive_by_value(o1);
    println!("< exit main()");
}
