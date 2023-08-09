# ratatui 두 번째 샘플 

Block을 중심으로 화면을 분할하여 배치하는 방법 위주로 설명한다. 
Paragraph, Style을 추가로 이해할 수 있다. 

```rust

use std::{
    error::Error,
    io::{stdout, Stdout},   // Stdout을 Write로 하여 CrosstermBackend를 만듦
    ops::ControlFlow,       // Continue와 Break를 처리할 수 있게 한다. 추가 이해 필요.
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Padding, Paragraph, Wrap,
    },
};

// These type aliases are used to make the code more readable by reducing repetition of the generic
// types. They are not necessary for the functionality of the code.
type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal);
    restore_terminal(terminal)?;

    if let Err(err) = result {
        eprintln!("{err:?}");
    }
    Ok(())
}

fn setup_terminal() -> Result<Terminal> {
    // Raw mode는 terminal 문서에 설명이 있다. 터미널들이 입력에 대한 기본 동작들이 있는데 
    // 이들 중 일부를 disable하여 별도로 제어할 수 있게 한다.
    enable_raw_mode()?;
    let mut stdout = stdout();
    // AlternateScreen은 현재 명령을 실행한 창이 아닌 다른 창을 연다는 뜻이다. 
    // 뭔가 실행 상태를 표시하고 깔끔하게 정리해서 이전의 터미널 상태로 돌아갈 수 있게 한다. 
    // execute!는 Crossterm의 매크로로 명령을 Write에 실행한다. 
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(mut terminal: Terminal) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn run(terminal: &mut Terminal) -> Result<()> {
    loop {
        terminal.draw(ui)?;

        // handle_events의 event::poll()에서 대기하도록 해서 CPU 100% 사용을 막는다. 
        if handle_events()?.is_break() {
            return Ok(());
        }
    }
}

fn handle_events() -> Result<ControlFlow<()>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(ControlFlow::Break(()));
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}

fn ui(frame: &mut Frame) {
    // immediate mode gui는 이렇게 함수로 그리는 인터페이스를 주로 사용한다. 
    // egui도 비슷한 구조로 widget들을 그린다. 

    // 화면을 여러 개로 나누는 area를 계산한다. 
    let (title_area, layout) = calculate_layout(frame.size());

    render_title(frame, title_area);

    // Paragraph를 생성한다.
    let paragraph = placeholder_paragraph();

    render_borders(&paragraph, Borders::ALL, frame, layout[0][0]);
    render_borders(&paragraph, Borders::NONE, frame, layout[0][1]);
    render_borders(&paragraph, Borders::LEFT, frame, layout[1][0]);
    render_borders(&paragraph, Borders::RIGHT, frame, layout[1][1]);
    render_borders(&paragraph, Borders::TOP, frame, layout[2][0]);
    render_borders(&paragraph, Borders::BOTTOM, frame, layout[2][1]);

    render_border_type(&paragraph, BorderType::Plain, frame, layout[3][0]);
    render_border_type(&paragraph, BorderType::Rounded, frame, layout[3][1]);
    render_border_type(&paragraph, BorderType::Double, frame, layout[4][0]);
    render_border_type(&paragraph, BorderType::Thick, frame, layout[4][1]);


    render_styled_block(&paragraph, frame, layout[5][0]);

    render_styled_borders(&paragraph, frame, layout[5][1]);

    render_styled_title(&paragraph, frame, layout[6][0]);

    render_styled_title_content(&paragraph, frame, layout[6][1]);

    render_multiple_titles(&paragraph, frame, layout[7][0]);

    render_multiple_title_positions(&paragraph, frame, layout[7][1]);

    render_padding(&paragraph, frame, layout[8][0]);

    render_nested_blocks(&paragraph, frame, layout[8][1]);
}

/// Calculate the layout of the UI elements.
///
/// Returns a tuple of the title area and the main areas.
fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Min(0)])
        .split(area);
    let title_area = layout[0];
    let main_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(4); 9])
        .split(layout[1])
        .iter()
        .map(|&area| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area)
                .to_vec()
        })
        .collect_vec();
    (title_area, main_areas)
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Block example. Press q to quit")
            .dark_gray()
            .alignment(Alignment::Center),
        area,
    );
}

fn placeholder_paragraph() -> Paragraph<'static> {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    // Paragraph를 만든다. wrap 옵션에 trim 하도록 설정한다. 
    // text.dark_gray()는 전경색을 지정한다. Stylize trait 내부의 함수로 매크로가 만들어진다.
    // text가 Stylize를 구현했기 때문에 동작한다. &str에서 Text로 into()가 가능하고 
    // 그래서, Text로 변환된 후에 dark_gray() 호출이 발생한다. 아직 미묘하다. 
    Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true })
}

fn render_borders(paragraph: &Paragraph, border: Borders, frame: &mut Frame, area: Rect) {
    // 블럭은 경계를 포함한다. 각 위젯에서 블럭 내부에 놓이는 구현을 갖는다.
    let block = Block::new()
        .borders(border)
        .title(format!("Borders::{border:#?}", border = border));

    // Paragraph의 render (Widget impl) 내부는 여러가지를 다양하게 사용한다. 
    // 텍스트들을 그린다는 기본 기능 정도를 이해한다. 
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_border_type(
    paragraph: &Paragraph,
    border_type: BorderType,
    frame: &mut Frame,
    area: Rect,
) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(border_type)
        .title(format!("BorderType::{border_type:#?}"));
    frame.render_widget(paragraph.clone().block(block), area);
}
fn render_styled_borders(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_style(Style::new().blue().on_white().bold().italic())
        .title("Styled borders");
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_styled_block(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    // 일관되게 style, border_style, title_style 등으로 스타일을 지정한다. 
    let block = Block::new()
        .borders(Borders::ALL)
        .style(Style::new().blue().on_white().bold().italic())
        .title("Styled block");
    frame.render_widget(paragraph.clone().block(block), area);
}

// Note: this currently renders incorrectly, see https://github.com/ratatui-org/ratatui/issues/349
fn render_styled_title(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Styled title")
        .title_style(Style::new().blue().on_white().bold().italic());
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_styled_title_content(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    // Paragraph처럼 Line은 텍스트를 포함하는 한 줄이다. 
    let title = Line::from(vec![
        "Styled ".blue().on_white().bold().italic(),
        "title content".red().on_white().bold().italic(),
    ]);
    let block = Block::new().borders(Borders::ALL).title(title);
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_multiple_titles(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Multiple".blue().on_white().bold().italic())
        .title("Titles".red().on_white().bold().italic());
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_multiple_title_positions(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title(
            Title::from("top left")
                .position(Position::Top)
                .alignment(Alignment::Left),
        )
        .title(
            Title::from("top center")
                .position(Position::Top)
                .alignment(Alignment::Center),
        )
        .title(
            Title::from("top right")
                .position(Position::Top)
                .alignment(Alignment::Right),
        )
        .title(
            Title::from("bottom left")
                .position(Position::Bottom)
                .alignment(Alignment::Left),
        )
        .title(
            Title::from("bottom center")
                .position(Position::Bottom)
                .alignment(Alignment::Center),
        )
        .title(
            Title::from("bottom right")
                .position(Position::Bottom)
                .alignment(Alignment::Right),
        );
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_padding(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Padding")
        .padding(Padding::new(5, 10, 1, 2));
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_nested_blocks(paragraph: &Paragraph, frame: &mut Frame, area: Rect) {
    let outer_block = Block::new().borders(Borders::ALL).title("Outer block");
    let inner_block = Block::new().borders(Borders::ALL).title("Inner block");
    // inner 함수로 계산한 inner area에 그려서 내부에 있는 것처럼 한다. 
    // 위젯간 포함 관계는 없다. 
    let inner = outer_block.inner(area);
    frame.render_widget(outer_block, area);
    frame.render_widget(paragraph.clone().block(inner_block), inner);
}

```