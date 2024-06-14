// {{## BEGIN procedural ##}}
fn rectangle_area(width: u32, height: u32) -> u32 {
    width * height
}

fn use_prims() {
    let rect_w = 12;
    let rect_h = 24;
    let area = rectangle_area(rect_w, rect_h);
    println!("The area of a {} x {} rectangle is {}", 
        rect_w, rect_h, area);
}
// {{## END procedural ##}}

// {{## BEGIN tuples ##}}
fn rect_tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn use_tuples() {
    let rect = (12, 24);
    let area = rect_tuple_area(rect);
    println!("The area of a {} x {} rectangle is {}", 
        rect.0, rect.1, area);
}
// {{## END tuples ##}}

// {{## BEGIN structs ##}}
struct RectStruct {
    width: u32,
    height: u32,
}

fn rect_area(rect: &RectStruct) -> u32 {
    rect.width * rect.height
}

fn use_struct() {
    let rect = RectStruct{
        width: 12, 
        height: 24,
    };
    let area = rect_area(&rect);
    println!("The area of a {} x {} rectangle is {}", 
        rect.width, rect.height, area);
}
// {{## END structs ##}}

// {{## BEGIN impls ##}}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn new(h: u32, w: u32) -> Self {
        Self { height: h, width: w }
    }
    fn square(size: u32) -> Self {
        Self { height: size, width: size } 
    }
}

fn use_object() {
    //let rect = Rectangle{ width: 12, height: 24, };
    let rect = Rectangle::new(12, 24);
    println!("The area of a {} x {} rectangle is {}", 
        rect.width, rect.height, rect.area());
    let rect = Rectangle::square(12);
    println!("The area of a {} x {} square is {}", 
        rect.width, rect.height, rect.area());
}
// {{## END impls ##}}

// {{## BEGIN traits ##}}
#[derive(Debug)] // this is an "attribute" using the Debug "trait"
struct DRectangle {
    width: u32,
    height: u32,
}

impl DRectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn use_trait() {
    let rect = DRectangle{
        width: 12, 
        height: 24,
    };
    dbg!(&rect);    // "[src/main.rs:95:5] &rect = DRectangle {
                    //        width: 12,
                    //        height: 24,
                    //  }" printed to stderr
    println!("The area of a {} x {} rectangle is {}", 
        rect.width, rect.height, rect.area());
}
// {{## END traits ##}}

fn main() {
    use_prims();
    use_tuples();
    use_struct();
    use_object();
    use_trait();
}
