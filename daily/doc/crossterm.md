# crossterm 

crossterm은 ANSI 명령으로 터미널 제어하는 기능을 갖춘 크레이트입니다. 

```rust
use std::io::{stdout, Write};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, 
    event,
};
fn main() -> std::io::Result<()> {
    // using the macro
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
    )?;

    // crossterm은 ANSI 명령으로 터미널을 제어한다. 
    // Command trait와 fmt::Write를 핵심으로 한다. 

    // or using functions
    stdout()
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Red))?
        .execute(Print("Styled text here."))?
        .execute(ResetColor)?;
    
    Ok(())
```

사용은 간단하고 rust text ui (tui)들의 백엔드로 사용합니다. 유용한 툴들을 간단하게 
만들기 위해 준비하고 있습니다. 

```rust
/// 아래 write_command_ansi 함수는 fmt::Write를 Write에 대해 구현하여 
/// 임의의 명령을 stdout과 같이 Write를 구현한 곳에 쓸 수 있게 한다. 
/// trait로 구조화 하는 또 다른 좋은 예시이다. 
fn write_command_ansi<C: Command>(
    io: &mut (impl io::Write + ?Sized),
    command: C,
) -> io::Result<()> {
    struct Adapter<T> {
        inner: T,
        res: io::Result<()>,
    }

    impl<T: Write> fmt::Write for Adapter<T> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.inner.write_all(s.as_bytes()).map_err(|e| {
                self.res = Err(e);
                fmt::Error
            })
        }
    }

    let mut adapter = Adapter {
        inner: io,
        res: Ok(()),
    };

    command
        .write_ansi(&mut adapter)
        .map_err(|fmt::Error| match adapter.res {
            Ok(()) => panic!(
                "<{}>::write_ansi incorrectly errored",
                std::any::type_name::<C>()
            ),
            Err(e) => e,
        })
}
```

위 코드는 fmt::Write를 구현하면 실행할 수 있는 명령으로 일관되게 crossterm을 구현한 
방법을 알 수 있는 코드입니다. 러스트는 이와 같이 반복적으로 트레이트, trait boound, 
확장으로 구조를 만들고 있습니다. 

구현이 없기 때문에 더 널리 쓰일 수 있는 trait는 OOP보다 나은 재사용성을 보여줍니다. 

