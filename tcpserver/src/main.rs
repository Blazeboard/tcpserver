use std::net::TcpListener;
use std::io::{ Read, Write };

fn main() {
    // 建立TcpListener, 与本地的3000端口绑定
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    // 通过for循环，不断监听客户端的TCP请求
    for stream in listener.incoming() { // incoming返回一个TcpStream的迭代器
        // 使用同名变量（变量遮蔽）stream获取TcpStream, 建立连接
        let mut stream = stream.unwrap();
        println!("Connection established!");

        // 设置buffer用于接收客户端发来的数据
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        // 将客户端发来的数据原样返回
        stream.write(&mut buffer).unwrap();
    }
}
