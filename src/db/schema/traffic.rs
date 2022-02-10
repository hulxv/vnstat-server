diesel::table! {
    traffic (id) {
        id -> Integer,
        interface -> Integer,
        date -> Date,
        rx -> Integer,
        tx -> Integer,
    }
}
