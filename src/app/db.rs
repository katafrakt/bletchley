extern crate rusqlite;

use self::rusqlite::Connection;
use qml::*;

pub struct Database {
    conn: Connection
}

impl Database {
    pub fn new(name: &str) -> Database {
        Database { conn: Connection::open(name).unwrap() }
    }

    pub fn ensure_structure(&self) {
        self.conn.execute("CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )", &[]).unwrap();
    }
}

Q_OBJECT!(
    pub Database as QDatabase {
        signals:
        slots:
            fn get_all_contacts();
            fn search_contacts(name: String);
        properties:
    }
);

impl QDatabase {
    pub fn get_all_contacts(&self) -> Option<&QVariant> { None }
    pub fn search_contacts(&self, name: String) -> Option<&QVariant> { None }
}
