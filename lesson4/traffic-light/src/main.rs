// 定义一个枚举类型，表示交通信号灯的三种状态
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义一个trait，包含一个返回时间的方法
trait TimeDuration {
    fn duration(&self) -> u32;
}

// 为交通信号灯实现TimeDuration trait
impl TimeDuration for TrafficLight {
    // 实现duration方法，返回一个u32类型的时间持续时间
    fn duration(&self) -> u32 {
        match self {
            // 如果是红灯，则返回40秒
            TrafficLight::Red => 40,
            // 如果是黄灯，则返回3秒
            TrafficLight::Yellow => 3,
            // 如果是绿灯，则返回55秒
            TrafficLight::Green => 55,
        }
    }
}

fn main() {
    // 创建三个不同的交通灯实例
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    // 调用duration方法来输出每个信号灯的持续时间
    println!("Red light duration: {} seconds", red.duration());
    println!("Yellow light duration: {} seconds", yellow.duration());
    println!("Green light duration: {} seconds", green.duration());
}
