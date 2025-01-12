## 三种多态技术示例

```rust
enum Shape1 {
    Circle { radius: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape1 {
    fn draw(&self) {
        match self {
            Shape1::Circle { .. } => println!("o"),
            Shape1::Triangle { .. } => println!("△"),
        }
    }

    fn area(&self) -> f64 {
        match self {
            Shape1::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape1::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

// 2&3. 共用的 Shape trait
trait Shape {
    fn draw(&self);
    fn area(&self) -> f64;
}

// 具体类型定义
struct Circle {
    radius: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

// Shape trait 的实现
impl Shape for Circle {
    fn draw(&self) {
        println!("o");
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Triangle {
    fn draw(&self) {
        println!("△");
    }

    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 使用泛型的静态分发函数
fn print_info<T: Shape>(shape: &T) {
    shape.draw();
    println!("Area: {}", shape.area());
}

fn main() {
    // 1. enum 方式的使用
    println!("1. Using enum:");
    let shapes1 = vec![
        Shape1::Circle { radius: 2.0 },
        Shape1::Triangle { base: 3.0, height: 4.0 },
    ];
    for shape in &shapes1 {
        shape.draw();
        println!("Area: {}", shape.area());
    }

    // 2. 使用泛型的静态分发
    println!("\n2. Using generic trait (static dispatch):");
    let circle = Circle { radius: 2.0 };
    let triangle = Triangle { base: 3.0, height: 4.0 };
    
    print_info(&circle);
    print_info(&triangle);

    // 3. 使用动态分发
    println!("\n3. Using dyn trait (dynamic dispatch):");
    let shapes3: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Triangle { base: 3.0, height: 4.0 }),
    ];
    
    for shape in &shapes3 {
        shape.draw();
        println!("Area: {}", shape.area());
    }
}
```