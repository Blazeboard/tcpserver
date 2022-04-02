use std::net::TcpListener;
use std::io::{ Read, Write };
use std::str;

fn main() {
    // 建立TcpListener, 与本地的3000端口绑定
    let listener = match TcpListener::bind("127.0.0.1:3000") {
        Ok(l) => l,
        Err(e) => panic!("{}", e),
    };
    println!("开始监听3000端口...");

    // 通过for循环，不断监听客户端的TCP请求
    for stream in listener.incoming() { // incoming返回一个TcpStream的迭代器
        // 使用同名变量（变量遮蔽）stream获取TcpStream, 建立连接
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        };
        println!("连接建立!");

         // 设置buffer用于接收客户端发来的数据，这里如果接收“hello”，
         // 那么buffer中除了前五个位置外，其余位置都是零，buffer的总大小仍为1024
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) => println!("客户端发送的字符数：{}", n), // 显示客户端发送过来的字符数
            Err(e) => println!("发生错误：{}", e),
        };
        println!("客户端发送的数据为：{}", str::from_utf8(&buffer).unwrap());
        // 将客户端发来的数据原样返回
        match stream.write(&mut buffer) {
            Ok(n) => println!("向客户端发送的字符数：{}", n), // 显示向客户端发送的字符数
            Err(e) => println!("发生错误：{}", e),
        };
    }
}
