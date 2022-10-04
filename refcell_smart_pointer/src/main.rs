use std::cell::RefCell;
use std::rc::Rc;
struct Database {
    max_connections: u32,
}

#[allow(dead_code)]
struct AuthService {
    db: Rc<RefCell<Database>>,
}

#[allow(dead_code)]
struct ContentService {
    db: Rc<RefCell<Database>>,
}

fn main() {
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100,
    }));

    let _auth_service = AuthService { db: Rc::clone(&db) };
    let _content_service = ContentService { db: Rc::clone(&db) };

    // This code compiles but panics at line 27
    let mut _r1 = db.borrow_mut();
    let _r2 = db.borrow_mut();
    db.borrow_mut().max_connections = 200;

    // RefCell allows interior mutability, bu should be used with care.
    // The ownership and borrow rules are somewhat relaxed to allows mutability of Rc references used with RefCell.
}
