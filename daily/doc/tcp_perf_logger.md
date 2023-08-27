# tcp perf - logger

ratatui를 ui로 상태를 표시하려고 했으나 서버 / 클라가 안정적으로 동작하지 않아 로거를 
사용하도록 교체했습니다. 아무래도 통신은 단순한 인터페이스로 구현하는 것이 나은 듯 합니다. 

러스트의 로그는 log::{trace, debug, info, warn, error}를 인터페이스로 하고 실제 로그 
동작을 연결하는 구현을 사용하여 로그를 관리하도록 합니다. 이번에는 log4rs를 사용했습니다. 

로거는 비동기 appender와 console에 로그 수준별로 색상 표시 등을 갖춰야 하는데 log4rs는 
아직 이런 기능이 부족한 것으로 보입니다. log4rs의 issue를 살펴보고 가능하면 참여하도록 
합니다. 

러스트에서 Result는 항상 처리하는 것이 좋습니다. 

```rust
    let read = stream.read_buf(&mut buf).await;
    match read {
       Ok(n) => {
            if n == 0 {
               error!("remote disconnected. peer:{}", peer);
               break;
            }
        }
        Err(e) => {
            error!("recv error:{}, peer:{}", e, peer);
            return Err(e.into());
        }
    }
```

위 코드에서 read에 대한 결과 처리가 없어 계속 무한 반복하는 오류를 만났습니다. 이와 같은 
경우는 매우 많을 듯 합니다. 매우 편리한 await?는 적절히 활용해야겠습니다. 

이제 여러 연결을 받아 echo 처리를 하는 데까지 구현했습니다. 에러와 통계를 잘 볼 수 있도록 
하고 점차 개선해 나가면 될 것 같습니다. 

아래는 서버 쪽 구현 코드입니다. 

```rust
use anyhow;
use bytes::BytesMut;
use std::net::SocketAddr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{io::stdout, thread, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Builder;
use log::{debug, info, warn, error};

pub fn run(args: super::Args) {
    let runtime = Builder::new_multi_thread()
        .enable_io()
        .thread_name("run-tcp")
        .build()
        .unwrap();

    // 소유권을 자세히 잘 정리하면 안정성에 큰 도움이 된다. 그것이 러스트다.
    let args2 = args.clone();

    let _result = runtime.block_on(run_tcp(&args));
}

async fn run_tcp(args: &super::Args) -> Result<(), anyhow::Error> {
    // listener를 만들고 accept를 하면 task로 각 클라 연결에 대해 echo 처리
    let listener = TcpListener::bind(&args.listen).await?;
    info!("listening on {}", args.listen);

    let running = true;

    while running {
        let result = listener.accept().await;
        match result {
            Ok((stream, remote_addr)) => {
                info!("accepted:{}", remote_addr);
                let echo_size = args.size.clone();

                tokio::spawn(async move {
                    let _ = run_stream(echo_size, stream).await;
                });
            }
            Err(e) => {
                error!("accept error:{:?}", e);
            }
        }
    }

    Ok(())
}

// write_buf()가 mut 참조를 필요로 한다. stream을 mut로 전달한다.
async fn run_stream(
    echo_size: u32,
    mut stream: TcpStream,
) -> Result<(), anyhow::Error> {
    let peer = stream.peer_addr().unwrap();
    let mut buf = BytesMut::with_capacity(echo_size as usize);
    let run = true;
    let mut echo_count = 0;

    while run {
        let read = stream.read_buf(&mut buf).await;
        match read {
            Ok(n) => {
                if n == 0 {
                    error!("remote disconnected. peer:{}", peer);
                    break;
                }
            }
            Err(e) => {
                error!("recv error:{}, peer:{}", e, peer);
                return Err(e.into());
            }
        }

        stream.write_buf(&mut buf).await?;
        buf.clear();

        echo_count += 1;
        debug!("echo: {}", echo_count);
    }

    Ok(())
}
```