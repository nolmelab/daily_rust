# Ractor Monte Carlo Example 

OSS 프로젝트에 참여하여 랙터를 게임에 맞는 액터 시스템으로 발전시키려고 하고 있습니다. 
아래는 두 번째 이슈 리포트를 한 내용입니다. 상호작용이 사람에게 매우 큰 동기부여가 
된다는 점을 새삼 느낍니다. 

## Issue - Monte Carlo Panic

```bash

    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/examples/monte_carlo`
thread 'tokio-runtime-worker' panicked at 'Failed to send message: Messaging(SendErr)', ractor/examples/monte_carlo.rs:105:14
thread 'tokio-runtime-worker' panicked at 'Failed to send message: Messaging(SendErr)', ractor/examples/monte_carlo.rs:105:14
thread 'thread 'tokio-runtime-workertokio-runtime-worker' panicked at '' panicked at 'Failed to send message: Messaging(SendErr)Failed to send message: Messaging(SendErr)', ', ractor/examples/monte_carlo.rsractor/examples/monte_carlo.rs::105105::1414
```

I had the exactly same problem on Windows 10 / Intel i5-9500 6 core machine.
Some more facts are:

If NUM_GAMES is decreased to around 10, the problem does not happen.
If myself.stop() is disabled in Game::handle(), then the problem does not happen.
I suspect that there are some race condition problem somewhere.

After running with debugger, I found that UnboundedSender fails at inc_num_messages():
```rust
    pub fn send(&self, message: T) -> Result<(), SendError<T>> {
        if !self.inc_num_messages() {
            return Err(SendError(message));
        }

        self.chan.send(message);
        Ok(())
    }
```
The only condition that inc_num_messages() returns false is when curr & 1 is 1. I don't understand the meaning of this line.
It seems that it should not happen since compare_exchange() in inc_num_messages() always add 2 to the current number.

Hope this helps.

