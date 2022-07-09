use std ::net::{TcpListener,TcpStream};
use std::thread;
use std::time;
use std::io;
use std::io::{Read,Write};
 
 
fn handle_client(mut stream: TcpStream) -> io::Result<()>{
    
    // 创建一个可变数组
    let mut buf = [0;512];
    // 单纯循环1000次
    for _ in 0..1000{
        // 读取stream数据
        let bytes_read = stream.read(&mut buf)?;
        // stream读取完毕，返回
        if bytes_read == 0{
            return Ok(());
        }
        
        // 将bytes_read里的数据返回stream中
        stream.write(&buf[..bytes_read])?;
        // 线程沉默1s
        thread::sleep(time::Duration::from_secs(1));
 
    }
    Ok(())
}
 
fn main() -> io::Result<()>{
    // 创建一个listener，绑定到本地的端口3000
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    // 创建一个thread的vec数组
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    
    // incoming()返回一个迭代器，监听listener上的连接。每个连接代表一个字节流(TcpStream)，数据在TcpStream
    // 上进行传送和接收
    for stream in listener.incoming() {
        let stream = stream.expect("failed");
        println!("accept from client:{:?}",&stream);
        
        // 创建子线程
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error))
        });
        // 将handle，即线程，放入数组中
        thread_vec.push(handle);
    }
    
    // 从数组中取出线程，阻塞主线程，直至所有的子线程执行完毕
    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
 
 
}