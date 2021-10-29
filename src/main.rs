use std::{thread, time};

enum 交通灯 {
    红灯,
    黄灯,
    绿灯,
}

trait Time {
    fn start(&self);
}

impl Time for 交通灯 {
    fn start(&self) {
        match self {
            交通灯::红灯 => {
                let third = time::Duration::from_secs(3);
                println!("红灯等待3秒");
                thread::sleep(third);
            }
            交通灯::绿灯 => {
                let second = time::Duration::from_secs(2);
                println!("绿灯等待2秒");
                thread::sleep(second);
            }
            交通灯::黄灯 => {
                let one = time::Duration::from_secs(1);
                println!("黄灯等待1秒");
                thread::sleep(one);
            }
        }
    }
}

fn main() {
    交通灯::红灯.start();
    交通灯::绿灯.start();
    交通灯::黄灯.start();
}

