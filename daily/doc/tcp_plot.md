# tcp - plot 

성능 데이터를 그래프로 표시할 필요가 있어 plotters를 살펴보았습니다. 

러스트는 아직 완전히 단단하지는 않으나 C++로는 잘 시도하지 않았거나 전문가들만 
사용하는 범용적인 유용한 라이브러리가 대단히 많습니다. 사용도 파이썬 처럼 사용하기 
편하다는 느낌이 강하게 듭니다. 

```rust
use anyhow;
use plotters::prelude::*;

pub fn plot<ITER>(name: &str, w: u32, h : u32, series : ITER, x_range: (f64, f64), y_range: (f64, f64)) -> Result<(), anyhow::Error> 
where ITER: Iterator<Item=(f64, f64)> {
    let path = format!("{}.png", name);

    let root_area = BitMapBackend::new(&path, (w, h)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption(name, ("sans-serif", 10))
        .build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;

    cc.configure_mesh()
        .x_labels(9)
        .y_labels(9)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    cc.draw_series(LineSeries::new(
        series,
        &RED))?
        .label("Plot")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    cc.configure_series_labels().border_style(&BLACK).draw()?;

    root_area.present().expect("Unable to write result to file");
    println!("Result has been saved to {}", path);

    Ok(())
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_1() {

        let x_axis = (-20.0..20.0).step(0.1);
        let series = x_axis.values().map(|x: f64| (x, x.sin()));

        let result = plot(
            "hello", 
            1024, 512, 
            series, 
            (-10.0, 10.0), 
            (-1.0, 1.0)
        );

        match result {
            Ok(_) => {
                println!("generated hello.png");
            }
            Err(e) => {
                println!("failed to generate:{:?}", e);
            }
        }
    }
}

```