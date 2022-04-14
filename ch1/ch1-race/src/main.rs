use std::thread;
fn main() {
    let mut data = 100;

    thread::spawn(|| {data = 500});
    thread::spawn(|| {data = 1000});

    println!("{}", data);
}

//线程数据安全
//同一内存数据分配到不同线程编译是不能通过的