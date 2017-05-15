use qml::*;
use app::db::{Database, QDatabase};

pub fn start() {
    let db = Database::new("bletchley.db");
    db.ensure_structure();
    let mut engine = QmlEngine::new();
    engine.load_file("qml/main.qml");
    let mut contacts = QListModel::new(&["name", "email"]);
    engine.set_property("contactsModel", &contacts.get_qvar());
    let qdb = QDatabase::new(db);
    engine.set_and_store_property("DB", qdb.get_qobj());
    engine.exec();
    engine.quit();
}
