use std::{
    collections::HashMap,
    io,
    sync::{Arc, Mutex},
};

use crossterm::{
    QueueableCommand,
    terminal::{Clear, ClearType},
};
use progress_bar::ProgressBar;

pub mod progress_bar;

type ProgressMap = Arc<Mutex<HashMap<usize, ProgressBar>>>;

fn render_progresses(progresses: &ProgressMap) -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    let mut last_render = std::time::Instant::now();

    loop {
        let map = progresses.lock().unwrap();
        if map.is_empty() {
            break;
        }
        //渲染频率
        if last_render.elapsed().as_millis() < 50 {
            continue;
        }
        stdout.queue(Clear(ClearType::All))?;
        //绘制所有进度条
        for (i, (id, progress)) in map.iter().enumerate() {
            let percent = if progress.total > 0 {
                (progress.value as f64 / progress.total as f64 * 100.0) as u8
            } else {
                0
            };
            println!(
                "Progress {}: [{}/{}] {:.2}%",
                id, progress.value, progress.total, percent
            );
        }
    }

    Ok(())
}
