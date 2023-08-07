# tcp LengthCodec

tokio에서 패킷 단위를 Frame으로 부른다. Frame을 수신하는 측을 Stream, 송신하는 측을 
Sink로 트레이트를 만들어 Framed에서 구현하고, 이들 트레이트를 확장한 StremExt와 SinkExt가 
Future를 돌려주는 함수들을 갖도록 하여 async 틀 안에서 동작하게 한다. 

StreamExt는 Framed::next()로 다음 Frame을 얻을 수 있도록 하고, SinkExt는 Framed::send()를 
할 수 있도록 한다. 

이와 같이 트레이트, 트레이트를 구현한 struct, 다시 이를 확장하는 트레이트 구현을 통해 
전체 구조를 잡는 방법을 러스트에서 자주 사용한다. 

Framed는 Codec과 함께 동작하는데 통신을 할 때 인코딩과 디코딩을 하게 되는데 이들 가공 
과정을 거친 단위를 Frame이라고 부르고, Codec은 이를 처리하는 기능을 뜻한다. 

{% code lineNumbers="true" %}
```rust
use bytes::{ Buf, BufMut, Bytes, BytesMut };
use std::io;
use tokio_util::codec::{ Decoder, Encoder };

// 특별히 상태를 갖지 않으므로 unit ()만 갖는다. 
struct LengthCodec(());

impl LengthCodec {
    pub fn new() -> Self {
        LengthCodec(())
    }
}

impl Decoder for LengthCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<BytesMut>, io::Error> {
        // 길이가 2바이트 보다 클 경우에 읽어서 페이로드 있는지 확인
        if buf.len() > 2 {
            // std::io::Cursor는 bytes::Buf를 구현하고, 다양한 get_* 함수들을 제공한다.
            let mut cbuf = io::Cursor::new(&buf);
            let payload_len = cbuf.get_i16_le() as usize;

            if buf.len() >= payload_len + 2 {
                buf.advance(2);
                // split_to()로 얻은 BytesMut는 메모리를 공유하고
                // Cow처럼 여러 곳에서 참조한 상태로 쓰기가 필요할 때 메모리를 할당해서
                // 사용하므로 안전하다. 여기서는 돌려준 BytesMut가 drop 된 후
                // 함수로 전달한 buf에 쓰게 되므로 추가 메모리 할당은 없다.
                return Ok(Some(buf.split_to(payload_len)));
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder<Bytes> for LengthCodec {
    type Error = io::Error;

    fn encode(&mut self, data: Bytes, buf: &mut BytesMut) -> Result<(), io::Error> {
        buf.reserve(data.len() + 2);
        // 길이를 2바이트 저장한다. 
        buf.put_i16_le(data.len() as i16);
        buf.put(data);
        Ok(())
    }
}

impl Encoder<BytesMut> for LengthCodec {
    type Error = io::Error;

    fn encode(&mut self, data: BytesMut, buf: &mut BytesMut) -> Result<(), io::Error> {
        buf.reserve(data.len() + 2);
        // 길이를 2바이트 저장한다. 
        buf.put_i16_le(data.len() as i16);
        buf.put(data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::LengthCodec;
    use anyhow;
    use bytes::{ Buf, BufMut, Bytes, BytesMut };
    use futures::SinkExt; // StreamExt는 tokio에 있는 걸 사용하고
    use tokio::net::{ TcpListener, TcpStream };
    use tokio_stream::StreamExt;
    use tokio_util::codec::Decoder;

    #[tokio::test]
    async fn impl_length_codec() {
        async fn server() -> anyhow::Result<()> {
            let listener = TcpListener::bind("127.0.0.1:7000").await?;

            loop {
                // Asynchronously wait for an inbound socket.
                let (stream, _) = listener.accept().await?;

                tokio::spawn(async move {
                    // LengthCodec이 구현한 Decoder가 framed 함수를 제공한다. 
                    let mut framed = LengthCodec::new().framed(stream);

                    loop {
                        // Framed<S, C>::next()는 StreamExt의 함수라고 한다. Future를 돌려주는.
                        // 마치 iterator처럼 사용할 수 있게 하므로 편리하다.
                        if let Some(message) = framed.next().await {
                            match message {
                                Ok(bytes) => {
                                    // SinkExt의 send를 사용한다.
                                    let _ = framed.send(bytes).await;
                                }
                                Err(err) => println!("Socket closed with error: {:?}", err),
                            }
                        }
                    }
                });
            }
        }

        async fn client() -> anyhow::Result<()> {
            let stream = TcpStream::connect("127.0.0.1:7000").await?;

            let mut buf = BytesMut::with_capacity(1024);

            let mut echo_count = 0;

            loop {
                let mut framed = LengthCodec::new().framed(stream);

                loop {
                    // BytesMut는 BUfMut를 구현하므로 다양한 put_* 함수들이 있어 사용 가능하다. 
                    buf.extend_from_slice(&[0; 1024]);
                    let send_buf = buf.split_to(1024);
                    let _ = framed.send(send_buf).await;

                    if let Some(message) = framed.next().await {
                        match message {
                            Err(err) => println!("Socket closed with error: {:?}", err),
                            Ok(_) => {
                                echo_count += 1;
                                println!("client: {}", echo_count);
                            }
                        }
                    }
                }
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

SinkExt와 StreamExt 코드를 보면 마음이 편안하지는 않다. C++이나 C#에서 구현하는 추상화 
방법과 다르고 추상화 수준도 달라서 그런 듯 하다. 

Stream은 Poll을 리턴하는 poll_next() 메서드를 갖고 StreamExt는 이것만 사용해서 모든 
필요한 기능을 구현하고 있다. 이는 AsyncRead와 AsyncReadExt의 관계와 비슷하다. 

SinkExt는 Sink를 확장(상속)하고 AsyncWrite, AsyncWriteExt와 비슷하게 구현된다. 처리 방향만
다르고 결국 StreamExt가 하고자 하는 일과 같다. 주고 받는 Item에 대한 변형이나 처리를 
함수형 언어처럼 구조화를 한다. 

