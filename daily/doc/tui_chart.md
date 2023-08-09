# ratatui 차트 

Layout을 Constraint를 지정하여 영역을 잡고 (chunks) 이들을 사용하여 Block 등의 
위젯을 그리는 것을 보여준다. 또 이들 위젯 중 하나로 Chart를 Dataset으로 표현하는 
방법을 보여준다. 

```rust
se std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

const DATA: [(f64, f64); 5] = [(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];
const DATA2: [(f64, f64); 7] = [
    (0.0, 0.0),
    (10.0, 1.0),
    (20.0, 0.5),
    (30.0, 1.5),
    (40.0, 1.0),
    (50.0, 2.5),
    (60.0, 3.0),
];

#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

struct App {
    signal1: SinSignal,
    data1: Vec<(f64, f64)>,
    signal2: SinSignal,
    data2: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl App {
    fn new() -> App {
        let mut signal1 = SinSignal::new(0.2, 3.0, 18.0);
        let mut signal2 = SinSignal::new(0.1, 2.0, 10.0);
        // signal1.by_ref()는 Iterator의 함수로 참조로 빌릴 수 있게 한다. 
        // signal1을 아래에서도 참조를 하므로 by_ref()가 없으면 Iterator로서 move 된다. 
        let data1 = signal1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        App {
            signal1,
            data1,
            signal2,
            data2,
            window: [0.0, 20.0],
        }
    }

    fn on_tick(&mut self) {
        for _ in 0..5 {
            self.data1.remove(0);
        }

        // extern()는 IntoIterator를 받는데 Iterator도 (trivially) IntoIterator이다. 
        self.data1.extend(self.signal1.by_ref().take(5));
        for _ in 0..10 {
            self.data2.remove(0);
        }

        // Iterator를 구현했다는 뜻은 take()와 같은 기능을 편리하게 사용할 수 있다는 뜻이다.
        self.data2.extend(self.signal2.by_ref().take(10));

        // window는 어디에 쓰이는가? X 축의 값 범위 지정에 사용한다. 
        // X 축이 이동을 하는 것처럼 처리한다. window로 하지 않고 xaxis_bounds로 했다면 더 나았을 듯
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // event::poll()에서 대기를 일부 한다. 
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    // Layout은 contraints로 Rect들의 덩어리를 만든다. 이렇게 계산된 Rect들에 
    // 뭔가를 그려서 레이아웃에 따라 그리도록 한다. 
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            // Into<Vec<Contraint>를 받는다. Constraint 슬라이스 참조로 만족.
            // 러스트에서 제네릭과 트레이트의 조합으로 강력한 기능을 얻는다. 
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        // area를 다른 것들로 해서 다양한 레이아웃을 만들 수 있다. 
        // chunks들이 많아지므로 약간 헷갈릴 수 있다. 
        .split(size);

    let x_labels = vec![
        // 스타일을 갖는 텍스트.  
        Span::styled(
            format!("{}", app.window[0]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("{}", (app.window[0] + app.window[1]) / 2.0)),
        Span::styled(
            format!("{}", app.window[1]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];

    let datasets = vec![
        Dataset::default()
            .name("data2")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&app.data1),

        Dataset::default()
            .name("data3")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .data(&app.data2),
    ];

    // bounds로 차트의 값 범위를 지정한다. 
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title("Chart 1".cyan().bold())
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(x_labels)
                .bounds(app.window),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(vec!["-20".bold(), "0".into(), "20".bold()])
                .bounds([-20.0, 20.0]),
        );
    f.render_widget(chart, chunks[0]);

    let datasets = vec![Dataset::default()
        .name("data")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&DATA)];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title("Chart 2".cyan().bold())
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 5.0])
                .labels(vec!["0".bold(), "2.5".into(), "5.0".bold()]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 5.0])
                .labels(vec!["0".bold(), "2.5".into(), "5.0".bold()]),
        );

    // chunks[1]에 그리도록 한다. 
    f.render_widget(chart, chunks[1]);

    let datasets = vec![Dataset::default()
        .name("data")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Scatter)
        .data(&DATA2)];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title("Chart 3".cyan().bold())
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 50.0])
                .labels(vec!["0".bold(), "25".into(), "50".bold()]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 5.0])
                .labels(vec!["0".bold(), "2.5".into(), "5".bold()]),
        );
    f.render_widget(chart, chunks[2]);
}

```