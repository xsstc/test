use std::io::{self,prelude::*,BufReader,Write};
use std::net::TcpStream;
use std::str;
 
 
fn main() -> io::Result<( )> {
    // 申明一个可变的 stream，使用TcpStream建立连接
    let mut stream = TcpStream::connect("127.0.0.1:3000")?;
    // 单纯循环1000次
    for _ in 0..1000 {
        // 申明一个可变的字符串 input
        let mut input = String::new();
        // 从输入中读取信息，保存在input中
        io::stdin().read_line(&mut input).expect("Failed to read");
        // 将input中信息转化为原始字节，写入stream中
        stream.write(input.as_bytes()).expect("failed to write");

        // 申明一个可变的reader，指向stream
        let mut reader =BufReader::new(&stream);
        // 申明一个buffer数组
        let mut buffer: Vec<u8> = Vec::new();
        // 使用stream上的read_until方法将stream上的信息读到buffer中，直至遇到b'\n'或EOF
        reader.read_until(b'\n',&mut buffer)?;
        // 将buffer中存储的信息转化为字符串后，输出
        println!("read form server:{}",str::from_utf8(&buffer).unwrap()); 
        // 输出空字符串
        println!("");
 
    }
    Ok(())
}
