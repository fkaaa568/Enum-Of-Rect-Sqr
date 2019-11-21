
#[derive(Debug)]
enum Shape{
    Rectangle{x:f64,y:f64},
    Circle(f64),
    Square(f64),
}

impl Shape {

    fn area(Shape:Shape)->f64{

        match Shape {
            Shape::Rectangle{x,y} => x*y as f64,
             Shape::Circle(x)=>3.14*x*x as f64,
             Shape::Square(x)=> x*x as f64,

             
         }
    }
}
fn main() {
    let ObjRect = Shape::Rectangle{x:2.30,y:32.0};
        //match ObjRect {
        //Shape::Rectangle{x,y} => println!("Area of Rectangle is {} ",x*y),
        //Shape::Circle(x) => println!("Area of circle{}",3.14*x*x),
        //Shape::Square(x) =>println!(" Area of Square{}", x*x),
        let objcir =  Shape::Circle(20.0);
        let objsqr = Shape::Square(24.0);
        println!("The Area of Circle is {} ",Shape::area(objcir));
        println!("The Area of Circle is {} ",Shape::area(ObjRect));
        println!("The Area of Circle is {} ",Shape::area(objsqr));

    }
