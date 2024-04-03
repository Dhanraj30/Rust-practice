

fn main() {
    trait Shape {
        fn new(length: f32, width: f32) -> Self;

        fn area(&self) -> f32;
    }
    struct Rectangle {
        length: f32,
        width: f32,
    }

    impl Shape for Rectangle{
        fn new(length: f32, width:f32) -> Rectangle{
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width ;
        }
    }

    let rec: Rectangle = Shape::new(10.0,20.0 );

    println!("Rec Area : {}", rec.area());
}