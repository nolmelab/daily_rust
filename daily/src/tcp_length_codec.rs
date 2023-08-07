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

                // 읽기만 해서는 안 되므로 쓰는 것도 필요하다.

                // 쓰는 건 어떻게 하는가?

                tokio::spawn(async move {
                    // LengthCodec이 구현한 Decoder가 framed 함수를 제공한다. 
                    let mut framed = LengthCodec::new().framed(stream);

                    loop {
                        // Framed<S, C>::next()는 StreamExt의 함수라고 한다. Future를 돌려주는.
                        // 마치 iterator처럼 사용할 수 있게 하므로 편리하다.
                        if let Some(message) = framed.next().await {
                            match message {
                                Ok(bytes) => {
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
