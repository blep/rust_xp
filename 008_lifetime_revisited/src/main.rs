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

impl Clone for Obj {
    fn clone(&self) -> Self {
        Obj::new( &self.name[..] )
    }

    fn clone_from(&mut self, source: &Self) { 
        self.name = source.name.clone();
    }
}


// Obj: copyable struct
// ////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
struct CopyObj {
    name: &'static str,
}

impl CopyObj {
    fn new( name: &'static str ) -> CopyObj {
        let o = CopyObj{ name: name };
        println!("+ Constructed CopyObj name={}", name);
        o
    }
}

impl Alive for CopyObj {
    fn alive(&self) {
        println!("= CopyObj name={} is alive", self.name);
    }
}

// Remarks: Drop cannot be implemented for "Copy" types.


// checking
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
    {
        let mut o1_clone = o1.clone();
        o1_clone.name = "main_clone".to_string();
        print_alive_by_value(o1); // pass by move semantic
        println!("returned from print_alive_by_value, o1 should have been destroyed");
        o1_clone.alive();
    }
    println!("exit sub-scope, o1_clone should have been destroyed");
    println!("");
    
    let mut o2 = CopyObj::new( "main_copyable" );
    o2.alive();
    print_alive_by_ref(&o2);
    print_alive_by_mut_ref(&mut o2);
    let mut o2_clone = o2.clone();
    o2_clone.name = "main_copyable_clone";
    print_alive_by_value(o2); // pass by copy semantic
    // because CopyObj implements the Copy trait, it can still be use
    o2.alive();
    o2_clone.alive();
    println!("");
    println!("< exit main()");
}
