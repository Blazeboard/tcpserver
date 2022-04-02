use std::net::TcpStream;
use std::io::{ Read, Write };
use std::str;

fn main() {
    // 与本地3000端口建立连接
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    // 向Steam中写入字符串Hello
    stream.write("Hello".as_bytes()).unwrap();

    // 创建buffer以获取服务器发来的数据，这里只获取前4个字符
    let mut buffer = [0; 4];
    stream.read(&mut buffer).unwrap();

    // 将获取到的字符串打印出来
    println!(
        "Response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
