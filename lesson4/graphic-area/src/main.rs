// 定义一个可以计算面积的trait
trait Area {
    fn area(&self) -> f64;
    fn shape_type(&self) -> &str;
}

// 定义一个圆形结构体，实现Area trait
struct Circle {
    radius: f64,
}

impl Area for Circle {
    // 实现area方法，计算圆形的面积
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    // 实现shape_type方法，返回图形类型
    fn shape_type(&self) -> &str {
        "Circle"
    }
}

// 定义一个三角形结构体，实现Area trait
struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    // 实现area方法，计算三角形的面积
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    // 实现shape_type方法，返回图形类型
    fn shape_type(&self) -> &str {
        "Triangle"
    }
}

// 定义一个正方形结构体，实现Area trait
struct Square {
    side: f64,
}

impl Area for Square {
    // 实现area方法，计算正方形的面积
    fn area(&self) -> f64 {
        self.side * self.side
    }

    // 实现shape_type方法，返回图形类型
    fn shape_type(&self) -> &str {
        "Square"
    }
}

// 定义一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数
fn print_area<T: Area>(shape: &T) {
    println!("The area of {} is {}", shape.shape_type(), shape.area());
}

fn main() {
    // 创建一个圆形实例，并调用print_area函数打印面积和图形类型
    let circle = Circle { radius: 3.0 };
    print_area(&circle);

    // 创建一个三角形实例，并调用print_area函数打印面积和图形类型
    let triangle = Triangle {
        base: 4.0,
        height: 5.0,
    };
    print_area(&triangle);

    // 创建一个正方形实例，并调用print_area函数打印面积和图形类型
    let square = Square { side: 2.0 };
    print_area(&square);
}
