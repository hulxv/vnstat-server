use super::{model::*, schema::*, *};
use chrono::{prelude::*, *};

use dirs::config_dir;
use std::fs::remove_file;

#[test]
async fn get_database_file_path() {
    let path = DatabaseFile::new().unwrap().path();
    println!("database path: {path}");
    assert_eq!(
        path,
        [
            config_dir().unwrap().to_str().unwrap(),
            "/vns/database/auth.db"
        ]
        .concat()
    )
}
#[test]
async fn create_database_if_not_exists() {
    DatabaseFile::new().unwrap().create_if_not_exists().unwrap();

    let path = DatabaseFile::new().unwrap().path();
    assert!(Path::new(&path).exists());
    remove_file(path).unwrap()
}

#[test]
async fn initlize_database() {
    use diesel::{sql_types::Text, QueryableByName};
    #[derive(Debug, QueryableByName, Clone, PartialEq)]
    struct Table {
        #[sql_type = "Text"]
        pub name: String,
    }

    let db = InitDatabase::connect().unwrap();

    db.init().unwrap();
    let tables = sql_query("SELECT name FROM sqlite_master WHERE type='table'")
        .load::<Table>(&db.conn)
        .unwrap();
    let excepted_tables = vec!["connections", "keys"];

    println!("{tables:#?}");

    for table in excepted_tables.into_iter() {
        assert!(tables.contains(&Table {
            name: table.to_owned()
        }))
    }
}
#[test]
async fn when_key_will_expires() {
    use app::Configs;
    let db = InitDatabase::connect().unwrap();

    db.init().unwrap();
    assert_eq!(
        Local::now()
            .checked_add_signed(Duration::days(
                Configs::init().unwrap().auth().key_expire_duration()
            ))
            .unwrap()
            .to_rfc2822(),
        Keys::generate_new_key(db.conn().clone(), "")
            .expires_at()
            .to_rfc2822()
    )
}

#[test]
async fn when_key_was_created() {
    let db = InitDatabase::connect().unwrap();
    let key = Keys::generate_new_key(db.conn(), "");
    println!("{:#?}", key);

    db.init().unwrap();
    assert_eq!(Local::now().to_rfc2822(), key.created_at().to_rfc2822())
}
#[test]
async fn create_new_key() {
    let db = InitDatabase::connect().unwrap();
    db.init().unwrap();

    // let (ip_addr, user_agent) = ("0.0.0.0", "USER-AGENT");

    // let connection = Connections::new(db.conn(), ip_addr, user_agent);
    // key.create(db.conn()).unwrap();

    let key = Keys::generate_new_key(db.conn(), "");
    key.create(db.conn()).unwrap();

    assert!(keys::table.load::<Keys>(db.conn()).unwrap().contains(&key))
}

#[test]
async fn create_new_connection() {
    let (ip_addr, user_agent) = ("0.0.0.0", "USER-AGENT");
    let db = InitDatabase::connect().unwrap();
    db.init().unwrap();
    let connection = Connections::new(ip_addr, user_agent);
    connection.create(db.conn()).unwrap();

    assert!(connections::table
        .load::<Connections>(db.conn())
        .unwrap()
        .contains(&connection))
}
#[test]
async fn create_new_connection_with_its_key() {
    let db = InitDatabase::connect().unwrap();
    db.init().unwrap();

    let (ip_addr, user_agent) = ("0.0.0.0", "USER-AGENT");

    let connection = Connections::new(ip_addr, user_agent);
    connection.create(db.conn()).unwrap();

    let key = Keys::generate_new_key(db.conn(), &connection.uuid());
    key.create(db.conn()).unwrap();
    assert!(keys::table.load::<Keys>(db.conn()).unwrap().contains(&key));
    assert!(connections::table
        .load::<Connections>(db.conn())
        .unwrap()
        .contains(&connection));

    for key in keys::table.load::<Keys>(db.conn()).unwrap() {
        if key.conn(db.conn()).is_some() {
            return;
        }
    }
    assert!(false)
}

#[test]
async fn validate_key() {
    let db = InitDatabase::connect().unwrap();
    db.init().unwrap();

    let connection = Connections::new("0.0.0.0", "USER-AGENT");
    connection.create(db.conn()).unwrap();

    let valid_key = Keys::generate_new_key(db.conn(), &connection.uuid());
    valid_key.create(db.conn()).unwrap();

    let invalid_keys = vec![
        // A key doesn't exist
        Keys::default(),
        // An exists key but connection uuid doesn't exist
        Keys::generate_new_key(db.conn(), "UNKNOWN")
            .create(db.conn())
            .unwrap(),
        // An exists key and connection uuid is exists but the key is expires
        Keys {
            id: match keys::table.load::<Keys>(db.conn()) {
                Err(_) => 0,
                Ok(keys) => match keys.last() {
                    Some(key) => key.id,
                    None => 0,
                },
            } + 1,
            value: "".to_owned(),
            expires_at: Local::now().to_rfc2822(),
            created_at: Local::now().to_rfc2822(),
            conn_uuid: connection.uuid(),
        }
        .create(db.conn())
        .unwrap(),
    ];

    assert_eq!(Keys::is_valid(db.conn(), &valid_key.value()), true);

    for k in invalid_keys.iter() {
        assert_eq!(Keys::is_valid(db.conn(), &k.value()), false);
    }
}

#[test]
async fn select_columns_from_connections_table() {
    let db = InitDatabase::connect().unwrap();
    db.init().unwrap();

    let connections = vec![
        Connections::new("0.0.0.0", "user_agent")
            .create(db.conn())
            .unwrap(),
        Connections::new("0.0.0.1", "user_agent")
            .create(db.conn())
            .unwrap(),
        Connections::new("0.0.0.1", "user_agent")
            .create(db.conn())
            .unwrap(),
        Connections::new("0.0.0.0", "user_agent")
            .create(db.conn())
            .unwrap(),
    ];

    let expected = vec![
        connections.first().unwrap().uuid(),
        connections.last().unwrap().uuid(),
    ];

    for uuid in expected {
        assert!(Connections::select(db.conn(), |c| c.ip_addr() == "0.0.0.0")
            .iter()
            .map(|c| c.uuid())
            .collect::<Vec<String>>()
            .contains(&uuid));
    }
}

#[test]
async fn select_columns_from_keys_table() {
    let db = InitDatabase::connect().unwrap();
    db.init().unwrap();
    let _1st_conn_uuid = Connections::new("0.0.0.0", "user_agent")
        .create(db.conn())
        .unwrap()
        .uuid();
    let _2nd_conn_uuid = Connections::new("0.0.0.0", "user_agent")
        .create(db.conn())
        .unwrap()
        .uuid();
    let keys = vec![
        Keys::generate_new_key(db.conn(), &_1st_conn_uuid)
            .create(db.conn())
            .unwrap()
            .value(),
        Keys::generate_new_key(db.conn(), &_2nd_conn_uuid)
            .create(db.conn())
            .unwrap()
            .value(),
        Keys::generate_new_key(db.conn(), &_1st_conn_uuid)
            .create(db.conn())
            .unwrap()
            .value(),
        Keys::generate_new_key(db.conn(), &_2nd_conn_uuid)
            .create(db.conn())
            .unwrap()
            .value(),
    ];

    let _1st_expected = vec![keys[0].clone(), keys[2].clone()];
    println!("1st: {_1st_expected:#?}");
    for key_value in Keys::select(db.conn(), |k| {
        if let Some(conn) = k.conn(db.conn()) {
            return conn.uuid() == _1st_conn_uuid;
        }
        false
    })
    .iter()
    .map(|k| k.value())
    {
        println!("{key_value}");
        assert!(_1st_expected.contains(&key_value));
    }

    let _2nd_expected = vec![keys[1].clone(), keys[3].clone()];

    println!("2nd: {_2nd_expected:#?}");
    for key_value in Keys::select(db.conn(), |k| {
        if let Some(conn) = k.conn(db.conn()) {
            return conn.uuid() == _2nd_conn_uuid;
        }
        false
    })
    .iter()
    .map(|k| k.value())
    {
        println!("{key_value}");
        assert!(_2nd_expected.contains(&key_value));
    }
}

#[test]
async fn select_columns_from_info_table() {
    let db = InitDatabase::connect().unwrap();
    db.init().unwrap();
    let expected = vec![format!("{}", DATABASE_VERSION)];
    for key_value in Info::select(db.conn(), |i| i.key() == "db_version")
        .iter()
        .map(|i| i.value())
    {
        assert!(expected.contains(&key_value));
    }
}
#[test]
async fn find_column_in_connections_table() {
    let db = InitDatabase::connect().unwrap();
    db.init().unwrap();
    let connections = vec![
        Connections::new("0.0.0.0", "user_agent")
            .create(db.conn())
            .unwrap()
            .uuid(),
        Connections::new("1.1.1.1", "user_agent")
            .create(db.conn())
            .unwrap()
            .uuid(),
        Connections::new("0.0.0.1", "user_agent")
            .create(db.conn())
            .unwrap()
            .uuid(),
        Connections::new("0.0.0.0", "user_agent")
            .create(db.conn())
            .unwrap()
            .uuid(),
    ];

    let expected = connections[1].clone();

    assert_eq!(
        expected,
        Connections::find(db.conn(), |c| c.ip_addr() == "1.1.1.1"
            && c.uuid() == expected)
        .unwrap()
        .uuid()
    );
}
