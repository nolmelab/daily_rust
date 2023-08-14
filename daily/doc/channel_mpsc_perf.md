# MPSC 채널의 성능 

actix에서 oneshot 채널을 필요할 때 생성해서 사용합니다. mpsc의 성능은 어느 정도일지 궁금하여 
전에 작업하던 소스에서 테스트를 했습니다. 간단하게 Instant를 사용하여 측정했습니다.

i7 windows pc에서 3개 코어를 100% 가까이 사용하면서 초당 5백만건을 송수신 할 수 있습니다. 
이는 TCP를 통한 처리 건수와 비슷하고 페이로드 크기에는 거의 영향을 받지 않으므로 대역폭은 
상당히 큽니다. 따라서, 대부분의 경우 충분한 성능을 제공합니다. 


```rust
use tokio::sync::mpsc;
use std::time::Instant;
use bytes::Bytes;

struct Message {
    payload: Box<Bytes>
}

impl Message {
    fn new(m: Box<Bytes>) -> Self {
        Self {
            payload: m
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1024); 

    let tx2 = tx.clone();

    println!("start.");

    let now = Instant::now();

    let data = Box::new(Bytes::from_static(&[0_u8;1024][..]));

    let j1 = tokio::spawn(async move {
        for _ in 0..10000000 {
            let _r = tx2.send(Message::new(data.clone())).await;
        }        
    });

    let j2 = tokio::spawn(async move {
        for _ in 0..10000000 {
            let _r = rx.recv().await; 
        }
    });

    let _ = j1.await;
    let _ = j2.await;

    println!("end. elapsed: {:?}", now.elapsed());

    // i7 윈도우 PC에서 초당 5백만개를 처리할 수 있다. 코어는 3개 정도를 100% 가까이 
    // 사용한다. 2 개는 송수신이고 하나는 스케줄링 관련된 것으로 보인다. 
}
```