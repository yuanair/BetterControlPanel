use test_macro::VisitFields;
#[derive(VisitFields)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Debug)]
#[derive(VisitFields)]
struct User2(i32, String, String);
#[derive(VisitFields)]
struct User3;

fn main() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    let user2 = User2(1, "Alice".to_string(), "alice@example.com".to_string());
    let user3 = User3;

    user.visit_fields();
    user2.visit_fields();
    user3.visit_fields();
}



