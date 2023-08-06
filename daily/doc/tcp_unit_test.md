# tcp 단위 테스트

tokio는 비동기 프로그래밍을 러스트에서 지원하는 런타임과 여러 도구들을 모아 놓은 크레이트입니다. C#의 async / await는 꽤 오래 봐왔지만 제대로 그 틀로 제품이나 서비스를 구현한 적이 없습니다. node.js도 Promise를 갖고 있지만 마찬가지입니다.

러스트는 비동기로 동시성 프로그래밍 문제를 해결하려고 시도했고 잘 정리된 것으로 보입니다. 물론 계속 발전하기는 하겠지만서도요.

게임 서버를 만들려면 통신이 기본이고 요즘은 거의 tcp만을 사용하기 때문에 tcp 통신 코드 작성이 그 시작입니다. 나중에는 클라/서버를 만들고 따로 실행해서 기능을 구현하겠지만 처음에 전체적인 사용법과 구조를 익힐 때는 단위 테스트에서도 동작하면 편리할 때가 많습니다.

현재 바이트 송수신, 길이를 갖는 프레임 처리, Json 프로토콜 처리, FlatBuffers 처리를 어떻게 할지 고민하고 있는 단계입니다. 이와 같은 내용을 고민하고 프로토콜 처리의 정확성을 테스트 하려면 단위 테스트 구조가 필요합니다.

{% code lineNumbers="true" %}
```rust
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

                    // split를 해서 recv / send 각각 task로 처리할 필요가 있다. 
                    // split에 대해 고민을 좀 더 한다. 
        
                    // In a loop, read data from the socket and write the data back.
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
        
            // In a loop, read data from the socket and write the data back.
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
```
{% endcode %}
