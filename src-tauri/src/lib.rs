use byteorder::{LittleEndian, ReadBytesExt};
use std::sync::atomic::{AtomicUsize, Ordering};
use tauri::ipc::Channel;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// 创建一个新的原子变量，初始值为0
static IS_STOP: AtomicUsize = AtomicUsize::new(0);
// https://v2.tauri.app/develop/calling-frontend/#channels
async fn read_fifo(on_event: Channel<&Vec<u8>>, lenght: u32) -> io::Result<()> {
    println!("开始异步读取");
    IS_STOP.store(0, Ordering::SeqCst);

    let mut f = File::open("/tmp/rtt1Fifo").await?;
    let mut buffer = vec![0; lenght as usize];

    // read up to the specified length
    loop {
        if IS_STOP.load(Ordering::SeqCst) == 0 {
            let bytes_read = f.read(&mut buffer).await?;
            if bytes_read > 0 {
                // 只发送实际读取到的字节
                on_event.send(&buffer).unwrap();
            }
        } else if IS_STOP.load(Ordering::SeqCst) == 1 {
            println!("暂停中");
        } else {
            // 检查是否收到了中止信号

            println!("接收到中止信号，停止读取。");
            break;
        }
    }
    Ok(())
}

#[tauri::command]
async fn start_fifo(on_event: Channel<&Vec<u8>>, length: u32) -> String {
    let _ = read_fifo(on_event, length).await.expect("Error reading fifo");
    format!("ok")
}

#[tauri::command]
fn read_fifo_stop() {
    // 设置新的值
    IS_STOP.store(2, Ordering::SeqCst);
}

#[tauri::command]
fn read_fifo_pause() {
    // 设置新的值
    IS_STOP.store(1, Ordering::SeqCst);
}

#[tauri::command]
fn read_fifo_continue() {
    // 设置新的值
    IS_STOP.store(0, Ordering::SeqCst);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            start_fifo,
            read_fifo_stop,
            read_fifo_pause,
            read_fifo_continue
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
