use std::net::TcpStream;
use std::io::{ Read, Write };
use std::str;

fn main() {
    // 与本地3000端口建立连接
    let mut stream = match TcpStream::connect("localhost:3000") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    // 向Steam中写入字符串Hello
    match stream.write("Hello".as_bytes()) {
        Ok(n) => println!("向服务端发送的字符数：{}", n), // 向服务端发送的字符数
        Err(e) => println!("{}", e),
    };

    // 创建buffer以获取服务器发来的数据，这里只获取前4个字符
    let mut buffer = [0; 4];
    match stream.read(&mut buffer) {
        Ok(n) => println!("从服务端接收到的字符数：{}", n), // 从服务端接收到的字符数
        Err(e) => println!("{}", e),
    };

    // 将获取到的字符串打印出来
    println!(
        "收到服务端的回复:{:?}",
        match str::from_utf8(&buffer) {
            Ok(str) => str,
            Err(e) => panic!("{}", e),
        }
    );
}
