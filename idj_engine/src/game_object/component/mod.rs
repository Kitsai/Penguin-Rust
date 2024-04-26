pub trait Component {
    fn update(&mut self, dt: f64);
    fn render(&self);
    fn start(&mut self);
    fn is(&self, name: &str) -> bool;
    fn notify_collision(&mut self, other: &mut dyn Component);
}