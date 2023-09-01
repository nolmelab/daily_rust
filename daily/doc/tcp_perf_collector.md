# collector 

## 또 다시 소유권 / 참조 

tokio::sync::mpsc::unbounded_channel()로 UnboundedReceiver, UnboundedSender를 
만들고 server::run_tcp와 run_stream에서 메세지를 받아 로거로 출력하는데까지 진행했다. 

Arc, Mutex로 둘러싸서 일단 빌드되고 동작하도록 했다. 러스트에서 소유권과 참조를 
이렇게 엄격하게 한 이유는 무엇일까? tokio를 쓰면서 쓰레드 여러 곳에서 참조를 갖거나
가변 참조를 가지면 버그의 온상이 될 수 있다는 점을 알 수 있다. 

단일 쓰레드일 경우에도 마찬가지로 보다 안심하고 쓸 수 있게 하고 그래서 더 안전하다. 

소유권이나 가변 / 비가변 참조를 좀 더 자세하게 나눠서 처리할 수 있도록 Sender, Receiver를 
분리한 것처럼 동일하게 분리할 필요가 있다. tokio 하에서 작업할 때는 특히 이 점을
더 잘 알아야 한다. 

## tokio의 성능 

tokio와 asio 같은 C++ 라이브러리 간 성능 비교를 한 정보는 없을까?

[asyncbench](https://github.com/patrykstefanski/async-bench/tree/master)에 간략한 에코
벤치마킹 비교 자료가 있다. asio, go 등 훌륭한 도구들은 큰 성능 차이가 있다고 보기 어렵다. 

통신 성능이 전체 앱의 성패를 좌우할 정도로 크지는 않다는 뜻이다. 

직접 경험하고 겪어 가면서 확인하면 된다. 실제 대역폭에 근접하는 정도의 부하를 줄 수 있는지, 
처리 지연은 어느 정도 되는지 확인하도록 한다. 

이제 Collector를 통해 파일에 정보를 남기고 리포트를 잘 만들면 tcperf는 한번 정리된다. 


