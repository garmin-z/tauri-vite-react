use std::net::{Ipv4Addr, UdpSocket};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

use serde::Serialize;
use tauri::{AppHandle, Manager, Runtime};
/// Global reference to the UDP socket so we can access it from multiple commands
static mut UDP_SOCKET: Option<Arc<Mutex<UdpSocket>>> = None;
static RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Clone)]
struct UdpResponse {
    ip: String,
    response: String,
}

#[tauri::command]
pub fn start_udp_broadcast<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    // 停止并等待之前的线程结束
    let _ = stop_udp_listener();

    // 绑定到任意IP和任意端口
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to address");
    // 设置广播选项
    socket
        .set_broadcast(true)
        .expect("set_broadcast call failed");
    let socket_arc = Arc::new(Mutex::new(socket));
    unsafe {
        UDP_SOCKET = Some(socket_arc.clone());
        RUNNING.store(true, Ordering::SeqCst);
    }
    let app_handle_clone = app.clone();

    // 将 UDP 操作放到另一个线程中
    thread::spawn(move || {
        // 目标广播地址和端口
        let broadcast_addr = Ipv4Addr::new(255, 255, 255, 255);
        let port = 18355;
        let msg = b"get_xarm_addr";

        let socket = socket_arc.lock().unwrap();
        // 发送广播消息
        socket
            .send_to(msg, (broadcast_addr, port))
            .expect("Couldn't send data");

        println!("Broadcast message sent!");

        // 设置接收超时时间
        socket
            .set_read_timeout(Some(Duration::from_millis(3000)))
            .expect("set_read_timeout call failed");

        // 接收响应
        let mut buf = [0; 1024];

        // 最大搜索时间s
        let mut timer = Instant::now();

        loop {
            // println!("Flag value=====>: {}", RUNNING.load(Ordering::SeqCst));
            let elapsed = timer.elapsed();
            if elapsed >= Duration::from_secs(60) {
                println!("Timeout reached, exiting loop");
                break;
            }

            if !RUNNING.load(Ordering::SeqCst) {
                break;
            }

            match socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    let response = String::from_utf8_lossy(&buf[..amt]).to_string();
                    println!("Received response from {}: {}", src, response);

                    let udp_response = UdpResponse {
                        ip: src.to_string().replace("18355", "18333"),
                        response,
                    };
                    app_handle_clone
                        .emit_all("udp_response", udp_response)
                        .expect("Failed to emit event");
                    timer = Instant::now();
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    println!("No more responses");
                    break;
                }
                Err(e) => {
                    println!("Error receiving data: {}", e);
                    // break;
                }
            }
        }

        println!("UDP broadcast thread stopped.");
    });

    Ok(())
}

#[tauri::command]
pub fn stop_udp_listener() -> Result<(), String> {
    println!("stop_udp_listener");
    unsafe {
        RUNNING.store(false, Ordering::SeqCst);
        if let Some(socket) = UDP_SOCKET.take() {
            drop(socket); // Close the socket
        }
    }

    Ok(())
}
