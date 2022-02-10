diesel::table! {
    interface (id) {
        id -> Integer,
        name -> Text,
        alias -> Text,
        active -> Integer,
        created -> Date,
        updated -> Date,
        rxcounter -> Integer,
        txcounter -> Integer,
        rxtotal -> Integer,
        txtotal -> Integer,
    }
}
