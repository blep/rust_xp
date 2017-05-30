
struct Vector2D {
    x: i32,
    y: i32,
}

struct Box {
	top_left: Vector2D,
	size: Vector2D,
}

impl Box {

    fn translated(self, direction: Vector2D) -> Box {
	    let (dx, dy) = (direction.x, direction.y);
		Box{ top_left: Vector2D{x: self.top_left.x+dx, y: self.top_left.y+dy}, 
		     size: self.size} // why cannot use .. self
	}

}

fn main() {
    let origin = Vector2D { x: 0, y: 0 }; // origin: Vector2D (immutable)

    println!("The origin is at ({}, {})", origin.x, origin.y);
	
	// origin.x = 5; // does not compile, mutability is declared on binding
	
	let mut origin2 = origin;
	origin2.x = 5;
    println!("The origin2 is at ({}, {})", origin2.x, origin2.y);

	let box1: Box = Box{ top_left: Vector2D{x: 5, y:7}, size: Vector2D{x: 2, y: 4} };
	println!("box1: x={}, y={}, width={}, heigh={}", box1.top_left.x, box1.top_left.y, box1.size.x, box1.size.y);
	
	let box2 = box1.translated( Vector2D{x: 1, y: 2} );
//	let Box { top_left: {x: bx, y: by}, size: {x: sx, y: sy} } = box2; // does not compile
	let Box { top_left: borig, size: bsize } = box2;
	println!("box2: x={}, y={}, width={}, heigh={}", borig.x, borig.y, bsize.x, bsize.y);
}
