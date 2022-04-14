fn greet_hello() {
    let korean = "안녕하세요";
    let english = "hello";
    let chinese = "你好";

    let regions = [korean, english, chinese];

    for region in regions.iter() {
        println!("{}", region);
    }
}

fn main() {
    greet_hello();
}

//utf8支持