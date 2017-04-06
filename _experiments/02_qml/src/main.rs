
#[macro_use]
extern crate qml;

fn main() {
    let mut engine = qml::QmlEngine::new();
    engine.load_file("src/main.qml");
    engine.exec();
    engine.quit();
}
