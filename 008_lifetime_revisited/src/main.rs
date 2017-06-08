use std::rc::Rc;
use std::borrow::Borrow;

trait Alive {
    fn alive(&self);
}

// Implement for single-threaded referenced counted pointer by forwarding to the underlying type.
impl<T: Alive> Alive for Rc<T> {
    fn alive(&self) {
        let obj: &T = self.borrow();
        obj.alive();
    }
}

// Implement for Box
impl<T: Alive> Alive for Box<T> {
    fn alive(&self) {
        let obj: &T = self.borrow();
        obj.alive();
    }
}


// Obj: simple struct
// ////////////////////////////////////////////////////////

struct Obj {
    name: String,
}

impl Obj {
    fn new( name: &str ) -> Obj {
        let o = Obj{ name: name.to_string() };
        let ptr: *const Obj = &o;
        println!("+ Constructed Obj name={} @{:?}", name, ptr);
        o
    }
}

impl Alive for Obj {
    fn alive(&self) {
        let ptr: *const Obj = self;
        println!("= Obj name={} is alive @{:?}", self.name, ptr);
    }
}

impl Drop for Obj {
    fn drop(&mut self) {
        let ptr: *const Obj = self;
        println!("- Dropping Obj name={} @{:?}", self.name, ptr);
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
        let ptr: *const CopyObj = &o;
        println!("+ Constructed CopyObj name={} @{:?}", name, ptr);
        o
    }
}

impl Alive for CopyObj {
    fn alive(&self) {
        let ptr: *const CopyObj = self;
        println!("= CopyObj name={} is alive @{:?}", self.name, ptr);
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
    
    // Non-copyable Obj (pass using move semantic)
    let mut o1 = Obj::new( "main" );
    o1.alive();
    print_alive_by_ref(&o1);
    print_alive_by_mut_ref(&mut o1);
    {
        let mut o1_clone = o1.clone();
        o1_clone.name = "main_clone".to_string();
        o1_clone.alive();
        print_alive_by_value(o1); // pass by move semantic
        println!("returned from print_alive_by_value, o1 should have been destroyed");
        o1_clone.alive();
    }
    println!("exit sub-scope, o1_clone should have been destroyed");
    println!("");
    
    // Copyable Obj (pass using copy)
    let mut o2 = CopyObj::new( "main_copyable" );
    o2.alive();
    print_alive_by_ref(&o2);
    print_alive_by_mut_ref(&mut o2);
    let mut o2_clone = o2.clone();
    o2_clone.name = "main_copyable_clone";
    o2_clone.alive();
    print_alive_by_value(o2); // pass by copy semantic
    // because CopyObj implements the Copy trait, it can still be use
    o2.alive();
    o2_clone.alive();
    println!("");
    
    // Ref counted Non-copyable obj
    let mut ro1 = Rc::new( Obj::new("rc_main") );
    ro1.alive();
    print_alive_by_ref(&ro1);
    print_alive_by_mut_ref(&mut ro1);
// Q: what trait is needed?!?
//    print_alive_by_mut_ref(&mut ro1.get_mut().unwrap());
    print_alive_by_value(ro1); // pass by move semantic
    println!("returned from print_alive_by_value, rc_main should have been destroyed");
    println!("");
    
    // Ref counted Non-copyable obj (keeping another reference on the ref counted pointer)
    let ro2 = Rc::new( Obj::new("rc_main2") );
    ro2.alive();
    {
        let ro2_clone = ro2.clone(); // "copy" the ref count ptr, increasing the number of reference
        print_alive_by_value(ro2); // pass by move semantic
        ro2_clone.alive();
    }
    println!("exit sub-scope, rc_main2 should have been destroyed");
    println!("");
    
    // Boxed non-copyable obj (hold a pointer to a dynamically allocated Obj)
    let mut bo1 = Box::new( Obj::new("box_main") );
    bo1.alive();
    print_alive_by_ref(&bo1);
    print_alive_by_mut_ref(&mut bo1);
    print_alive_by_ref(&*bo1);
    {
        let mut bo1_clone = bo1.clone(); // allocate a new clone of the pointed value
        (*bo1_clone).name = "box_main_clone".to_string();
        bo1_clone.alive();
        print_alive_by_value(bo1); // pass by move semantic
        println!("returned from print_alive_by_value, box_main should have been destroyed");
    }
    println!("exit sub-scope, box_main_clone should have been destroyed");
    println!("");
    
    println!("< exit main()");
}
