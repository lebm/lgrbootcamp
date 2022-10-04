use std::rc::Rc;
struct Database {}

#[allow(dead_code)]
struct AuthService {
    db: Rc<Database>,
}

#[allow(dead_code)]
struct ContentService {
    db: Rc<Database>,
}

fn main() {
    // Rc can only be used in single threaded applocations.
    // In multi threaded applications Arc should be used instead.
    let db = Rc::new(Database {});

    // Rc::clone just increments the refcount count.
    let _auth_service = AuthService { db: Rc::clone(&db) };
    let _content_service = ContentService { db: Rc::clone(&db) };
}
