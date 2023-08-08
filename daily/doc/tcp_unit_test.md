# tcp 단위 테스트

tokio는 비동기 프로그래밍을 러스트에서 지원하는 런타임과 여러 도구들을 모아 놓은 크레이트입니다. 
C#의 async / await는 꽤 오래 봐왔지만 제대로 그 틀로 제품이나 서비스를 구현한 적이 없습니다. 
node.js도 Promise를 갖고 있지만 마찬가지입니다.

러스트는 비동기로 동시성 프로그래밍 문제를 해결하려고 시도했고 잘 정리된 것으로 보입니다. 
물론 계속 발전하기는 하겠지만서도요.

게임 서버를 만들려면 통신이 기본이고 요즘은 거의 tcp만을 사용하기 때문에 tcp 통신 코드 
작성이 그 시작입니다. 나중에는 클라/서버를 만들고 따로 실행해서 기능을 구현하겠지만 처음에 
전체적인 사용법과 구조를 익힐 때는 단위 테스트에서도 동작하면 편리할 때가 많습니다.

현재 바이트 송수신, 길이를 갖는 프레임 처리, Json 프로토콜 처리, FlatBuffers 처리를 어떻게 
할지 고민하고 있는 단계입니다. 이와 같은 내용을 고민하고 프로토콜 처리의 정확성을 테스트 
하려면 단위 테스트 구조가 필요합니다.

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
```
{% endcode %}

전체 코드를 부분으로 나눠서 살펴봅니다. 

{% code lineNumbers="true" %}
```rust
#[cfg(test)]
mod tests {
    use anyhow;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
```
{% endcode %}

anyhow는 Error 처리를 편리하기 하도록 Box<dyn Error>와 같은 기능을 하는 Result<T>를 
제공합니다. thiserror 크레이트는 더 정확하게 에러 처리를 할 수 있는 편의 기능을 제공합니다. 
두 크레이트를 한 사람이 만들었다고 합니다. 

AsyncReadExt아 AsyncWriteExt는 Future를 돌려주는 편의 함수들을 IO에 대해 제공합니다. 
AsyncRead, AsyncWrite 트레이트에 대해 동작하도록 되어 있습니다. TcpStream이 AsyncRead, 
AsyncWrite를 구현하고 있어 AsyncReadExt와 AsyncWriteExt의 편의 기능을 사용할 수 있도록 합니다. 


{% code lineNumbers="true" %}
```rust
    #[tokio::test]
    async fn listen_connect() {
```
{% endcode %}

`#[tokio::test]`는 `async` 함수를 테스트로 실행할 수 있도록 한다. cargo test로 실행 가능하고 
tokio 런타임에서 실행합니다. 

{% code lineNumbers="true" %}
```rust
  async fn server() -> anyhow::Result<()> {
      // listener를 bind까지 해서 만들어준다. 
      let listener = TcpListener::bind("127.0.0.1:7000").await?;

      loop {
          // accept를 비동기로 실행한다. 생략된 _는 리모트 TcpSocketAddr이다. 
          let (mut stream, _) = listener.accept().await?;
  
          // tokio task를 만들어 stream: TcpStream을 처리한다. 
          tokio::spawn(async move {
              let mut buf = vec![0; 1024];

              // 읽고 그대로 돌려주는 echo 기능을 실행한다. 
              loop {
                  let n = stream 
                      .read(&mut buf)     // buf로 읽어 들인다. AsyncReadExt의 Future를 돌려준다.
                      .await              // Future에 대해 await를 한다. 
                      .expect("failed to read data from socket");
  
                  // n은 받은 길이이고 0이면 상대방이 소켓을 닫은 경우이다.
                  if n == 0 {
                      return anyhow::Ok(()); // anyhow에서 Ok 함수를 제공한다. 
                  }
  
                  stream 
                      .write_all(&buf[0..n])  // write_all도 AsyncReadExt의 함수로 Future를 돌려준다.
                      .await                  // Future에 대해 await를 한다.  
                      .expect("failed to write data to socket");
              }
          });
      }
  } 
```
{% endcode %}


{% code lineNumbers="true" %}
```rust
        async fn client() -> anyhow::Result<()> {
            // TcpStream::connect() 함수로 stream을 만든다.
            let mut stream = TcpStream::connect("127.0.0.1:7000").await?;

            let mut buf = vec![0; 1024];

            let mut echo_count = 0;
        
            loop {
                // 0으로 초기화된 버퍼 내용을 쓴다. 
                stream 
                    .write_all(&buf[0..1024])   // AsyncStreamExt의 함수이다. 
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

        // server()가 돌려준 Future에 대해 await하는 태스크를 만든다.
        let task_1 = tokio::spawn(async {
            let _ = server().await;
        });

        // client()가 돌려준 Future에 대해 await하는 태스크를 만든다.
        let task_2 = tokio::spawn(async {
            let _ = client().await;
        });

        // 각 태스크를 대기한다. 
        let _ = task_1.await;
        let _ = task_2.await;
    }
}
```
{% endcode %}

위 코드는 종료하지 않는데 echo 개수를 지정해서 종료하게 할 수 있습니다. 
buf 내용과 AsyncReadExt, AsyncWriteExt 함수들을 활용하여 추가 실험들을 할 수 있습니다. 

단위 테스트로 통신이 가능하게 하면 여러가지 조작을 쉽게 해볼 수 있습니다. 

이와 같은 기능을 C++이나 C#에서 만든다고 하면 얼마나 쉽게 할 수 있을까요? 

