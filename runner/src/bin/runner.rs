use took::Timer;

pub fn main() {
    let timer = Timer::new();
    runner::jobs().iter().for_each(|j| j.0());
    timer.took().describe("everything");
}
