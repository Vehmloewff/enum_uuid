use enum_uuid::EnumUuid;

#[derive(Debug, EnumUuid)]
enum Greetings {
    Hello,
    Hi,
    Peace,
}

fn main() {
    let id = Greetings::Peace.to_id();

    println!("{:?}", Greetings::from_id(id));
}
