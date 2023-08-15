# tcp perf - ratatui 

tcp 성능 측정은 tokio 런타임에 비동기로 실행합니다. 이 과정과 ratatui를 통한 처리가 
자연스럽게 통합되어야 합니다. 

channel이 가장 적합한 선택으로 보입니다. tcp 동작은 tokio 런타임 생성 후 block_on으로 실행합니다. 
이와 같이 하고 채널을 통해 정보를 받으면 됩니다. 

채널은 Future로 구현된 tokio의 채널 대신에 std::sync::mpsc::Channel을 사용합니다. 

```rust
    let (tx, rx) = std::sync::mpsc::channel();

    run_tcp(tx);

    run_ui(title, rx);
```

run_tcp에서 block_on()으로 실행하고, run_ui()는 메인 쓰레드에서 실행합니다. 



