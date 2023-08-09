# ratatui 첫 샘플 

ratatui는 tui 프로젝트가 아카이브 된 후 포크하여 개발을 지속하는 프로젝트이다. 
라따뚜이는 anyone can cook 대사로 유명한 쥐가 요리하는 애니메이션이다. 

crossterm을 백엔드로 Paragraph 하나를 그리는 예제이다. 
전체적인 ratatui의 동작 흐름을 이해할 수 있다. 

```rust
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};

use std::{
    error::Error,
    io::stdout,
    thread,
    time::{Duration, Instant},
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let stdout = stdout();
    // execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);

    // Terminal은 buffers: [Buffer; 2]를 갖는다. 이중 버퍼링으로 보인다.
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let now = Instant::now();
    while now.elapsed() < Duration::from_secs(5) {
        // Frame이 Terminal을 mut 참조로 갖는다.
        // render_widget<W> 함수에서 widget.render()를 현재 버퍼와 area에 그리도록 호출한다.
        // StatefulWidget과 Widget이 있고, render_widget과 render_stateful_widget으로 나눠서 호출한다.
        terminal.draw(|f| f.render_widget(Paragraph::new("termwiz example"), f.size()))?;

        // CompletedFrame<'a>는 직전에 그리기를 완료한 프레임이다. buffer:&'a Buffer를 
        // area와 함께 갖는다. 
        thread::sleep(Duration::from_millis(250));

        // Widget은 trait로 render 함수 하나만 구현한다. area, buf를 갖는다. 
        // List Widget을 보면 ListItem을 갖고 있고 ListState도 갖는다. 
        // StatefulWidget과 Widget을 모두 구현한다. 
        // 그려지는 영역은 Block으로 구성한다. 
    }

    terminal.show_cursor()?;
    terminal.flush()?;
    Ok(())
}

// core::macros::matches! 여러 개 중의 하나를 매칭할 경우에 대해 사용 
// uint_macros.rs에 checked_add, saturated_sub와 같은 여러 함수들이 있다.  
// rust 코드는 그 동안 읽었던 코드들 중 실용적으로 가장 단단하고 멋지다. 하스켈은 아름다워 보인다. 
//  
```

ratatui는 immediate mode gui의 하나이다. unity 3d engine의 툴 제작에 사용하여 유명해진 
imgui가 immediate mode gui의 약자이다. 프로그래머가 툴을 만들기에 괜찮은 접근이다. 

매번 widget 생성, 렌더링을 하는 방식이다. window 관리자가 없고 위젯 중심이기 때문에 
프레임워크 크기가 작고 사용이 수월한 측면이 있다. 

