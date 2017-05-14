extern crate qml;

use self::qml::*;

pub fn start() {
    let mut engine = QmlEngine::new();
    engine.load_file("qml/main.qml");
    let mut contacts = QListModel::new(&["name", "email"]);
    contacts.append_row(qvarlist!["John", "john@doe.com"].into_iter());
    contacts.append_row(qvarlist!["Jane", "jane@doe.com"].into_iter());
    engine.set_property("contactsModel", &contacts.get_qvar());
    engine.exec();
    engine.quit();
}
