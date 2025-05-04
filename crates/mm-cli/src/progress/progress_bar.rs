#[derive(Debug, Clone)]
pub struct ProgressBar {
    pub name: String,
    pub value: usize,
    pub total: usize,
    pub speed: usize,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        ProgressBar {
            value: 0,
            total,
            name: todo!(),
            speed: 0,
        }
    }

    pub fn increment(&mut self) {
        if self.value < self.total {
            self.value += 1;
        }
    }

    pub fn display(&self) {
        let percentage = (self.value as f64 / self.total as f64) * 100.0;
        println!(
            "Progress: [{}/{}] {:.2}%",
            self.value, self.total, percentage
        );
    }
}
