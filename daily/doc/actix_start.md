# actix 공부의 시작 

```rust
use actix::prelude::*;

// this is our Message
// we have to define the response type (rtype)
#[derive(Message)]
#[rtype(result = "usize")]
struct Sum(usize, usize);

// Actor definition
struct Calculator;

// actix도 문서화가 잘 되어 있다. https://actix.rs/docs 
// Actor, Context, Runtime, Arbiter, Message, Handler  
// Actor 프레임워크와 비동기 실행을 위한 요소들을 갖는다. 
impl Actor for Calculator {
    type Context = Context<Self>;
}

// now we need to implement `Handler` on `Calculator` for the `Sum` message.
// Handler는 어떻게 찾을까? Handler는 AddressSender의 send() 함수 빌드에 포함된다. 
// 
// pub fn send<M>(&self, msg: M) -> Result<OneshotReceiver<M::Result>, SendError<M>>
// where
//  A: Handler<M>,
//  A::Context: ToEnvelope<A, M>,
//  M::Result: Send,
//  M: Message + Send, 
// 위 호출에서 Handler<M>이 Handler<Sum>이다. 
impl Handler<Sum> for Calculator {
    type Result = usize; // <- Message response type

    fn handle(&mut self, msg: Sum, _ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}

pub async fn main() {
    // addr에 Mabilbox 정보가 포함되어 있다. 여러 호스트에 분산 가능할까? 
    // Akka나 Orleans 처럼 분산 처리까지 포함하면 더 큰 시스템이 될 수도 있다. 
    let addr = Calculator.start();

    // send()와 res에 대한 await로 동작한다. 비동기로 더 처리할 수도 있다. 
    // AddressSender 구현에서 그 동안 잘 몰랐던 parking을 알았다. Suspend에 해당하고 
    // park task queue로 넘겨서 관리한다. 

    // oneshot_channel()을 만들어 넘기고, Request Future로 돌려준다. 
    // send() 흐름에 연관된 Mailbox, Oneshot channel, Request Future의 구현 등은 
    // async로 뭔가를 할 때 매우 중요한 구조이고 도구들이다. 다른 건 대부분 괜찮고 Pin이 
    // 아직 걸린다.
    // 
    // Oneshot 채널로 정보를 전달하고 Waker로 깨우는 방법은 혼자서 고민한 방법 중 하나이다. 
    // Actix를 더 깊게 이해하고 많이 배우면 많은 성장이 가능하다. 
    let res = addr.send(Sum(10, 5)).await; // <- send message and get future for result

    match res {
        Ok(result) => println!("SUM: {}", result),
        _ => println!("Communication to the actor has failed"),
    }

    // Arbiter와 SyncArbiter가 쓰레드를 만들고 task를 실행하는 기능을 가지므로 
    // 중요한 기능을 하는 요소이다. 어떻게 구현했는지, tokio와 관계는 어떤지 파악하고 
    // 이해하고 게임 서버 설계와 구현에 활용하도록 한다. 
}

```

actix 구현은 개념이 명확하고 문서화가 잘 되어 있습니다. 참 잘 구현하고 모범적으로 문서화를
했으므로 많이 배워야겠습니다. 
