use b3_platform::{ActiveApplication, Application, Event};

fn main() {
    let app = Application::new();
    app.run(|app: &mut ActiveApplication, event: Event| println!("{:?}", event));
}
