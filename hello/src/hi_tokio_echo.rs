use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use tokio::select;
use tokio::sync::broadcast;

pub struct Handle {
    stream: TcpStream,
    tx: broadcast::Sender<u8>,
}

async fn echo() {
    let addr = "127.0.0.1:8089".to_string();
    let listener = TcpListener::bind(&addr).await.unwrap();
    let (tx, mut rx) = broadcast::channel::<u8>(1);
    loop {
        select! {
            r = listener.accept() => {
                let (stream, _) = r.unwrap();
                let mut handle = Handle{
                    stream,
                    tx: tx.clone()
                };
                tokio::spawn(async move {
                    let mut buf = vec![0; 1024];
                    loop {
                        let n = handle.stream.read(&mut buf).await.unwrap();
                        if n == 0 {
                            return;
                        }
                        if buf[0] == 'q' as u8 {
                            handle.tx.send('q' as u8).unwrap();
                            return;
                        }
                        handle.stream.write_all(&buf[0..n]).await.unwrap();
                    }
                });
            },
            _ = rx.recv() => {
                return;
            }
        }
    }
}

async fn client(){
    let addr = "127.0.0.1:8089".parse().unwrap();
    let socket = TcpSocket::new_v4().unwrap();
    let mut stream = socket.connect(addr).await.unwrap();
    stream.write_u8('a' as u8).await.unwrap();
    let mut buf = vec![0; 128];
    let n = stream.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..n], b"a");
    stream.write_u8('q' as u8).await.unwrap();
    return;
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_run_echo(){
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                tokio::spawn(echo());
                tokio::time::sleep(Duration::from_secs(3)).await;
                tokio::spawn(client());
            });
    }
}