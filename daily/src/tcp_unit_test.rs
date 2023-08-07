#[cfg(test)]
mod tests {
    use anyhow;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn listen_connect() {

        async fn server() -> anyhow::Result<()> {
            let listener = TcpListener::bind("127.0.0.1:7000").await?;

            loop {
                // Asynchronously wait for an inbound socket.
                let (mut stream, _) = listener.accept().await?;
        
                tokio::spawn(async move {
                    let mut buf = vec![0; 1024];

                    loop {
                        let n = stream 
                            .read(&mut buf)
                            .await
                            .expect("failed to read data from socket");
        
                        if n == 0 {
                            return anyhow::Ok(()); // anyhow에서 Ok 함수를 제공. 왜 필요하지?
                        }
        
                        stream 
                            .write_all(&buf[0..n])
                            .await
                            .expect("failed to write data to socket");
                    }
                });
            }

        } 

        async fn client() -> anyhow::Result<()> {
            let mut stream = TcpStream::connect("127.0.0.1:7000").await?;

            let mut buf = vec![0; 1024];

            let mut echo_count = 0;
        
            loop {
                stream 
                    .write_all(&buf[0..1024])
                    .await
                    .expect("failed to write data to socket");

                let n = stream
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return anyhow::Ok(()); // anyhow에서 Ok 함수를 제공. 왜 필요하지?
                }

                echo_count += 1;
                println!("client: {}", echo_count);
            }
        }

        let task_1 = tokio::spawn(async {
            let _ = server().await;
        });

        let task_2 = tokio::spawn(async {
            let _ = client().await;
        });

        let _ = task_1.await;
        let _ = task_2.await;
    }
}