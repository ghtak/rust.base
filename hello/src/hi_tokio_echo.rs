use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use tokio::select;
use tokio::sync::broadcast;

pub struct Handle {
    stream: TcpStream,
    tx: broadcast::Sender<u8>,
}

async fn echo_handler(mut handle: Handle) {
    tokio::spawn(async move {
        let mut buf = vec![0; 1024];
        loop {
            let n = handle.stream.read(&mut buf).await.unwrap();
            println!("read {}", std::str::from_utf8(&buf[..n]).unwrap());
            if n > 0 {
                handle.stream.write_all(&buf[0..n]).await.unwrap();
                if buf[0] == 'q' as u8 {
                    handle.tx.send('q' as u8).unwrap();
                    return;
                }
            } else {
                return;
            }
        }
    });
}

async fn echo() {
    let addr = "127.0.0.1:8089".to_string();
    let listener = TcpListener::bind(&addr).await.unwrap();
    let (tx, mut rx) = broadcast::channel::<u8>(1);
    loop {
        select! {
            r = listener.accept() => {
                let (stream, _) = r.unwrap();
                echo_handler(Handle{ stream, tx: tx.clone()}).await;
            },
            _ = rx.recv() => {
                return;
            }
        }
    }
}

async fn client() {
    tokio::time::sleep(Duration::from_secs(1)).await;
    let addr = "127.0.0.1:8089".parse().unwrap();
    let socket = TcpSocket::new_v4().unwrap();
    let mut stream = socket.connect(addr).await.unwrap();
    for c in "hello tokio q".chars() {
        stream.write_u8(c as u8).await.unwrap();
        assert_eq!(stream.read_u8().await.unwrap(), c as u8);
    }
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_echo() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                tokio::join!(echo(), client());
            });
    }
}
