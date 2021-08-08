//!
//! データベース接続、および関連操作を提供します。
//!

use uuid::Uuid;

use super::db;

fn generate_uuid4() -> String {
	let uuid = Uuid::new_v4();
	let l1 = uuid.hyphenated().to_string();
	let l2 = format!("{}", uuid);
	assert_eq!(l1, l2);
	return l2;
}

pub struct Service {
	// データベース接続
	_connection: Option<sqlite::Connection>,
}

impl Service {
	pub fn init(&mut self) -> Result<(), sqlite::Error> {
		let _connection = self.open();
		return Ok(());
	}

	fn open(&mut self) -> &mut sqlite::Connection {
		// 既に開いている場合は既存の接続を返します。
		if self._connection.is_some() {
			return self._connection.as_mut().unwrap();
		}

		// メモリ上の仮想データベースを開きます。
		self._connection = Some(sqlite::open(":memory:").unwrap());
		let connection = self._connection.as_mut().unwrap();
		return connection;
	}

	pub fn dump(&mut self) -> Result<(), sqlite::Error> {
		let connection = self.open();
		let mut statement = connection.prepare("SELECT * FROM USERS")?;
		while let sqlite::State::Row = statement.next()? {
			println!(
				"id={}, name={}",
				statement.read::<String>(0)?,
				statement.read::<String>(1)?
			);
		}
		return Ok(());
	}

	pub fn new() -> Service {
		let s = Service { _connection: None };
		return s;
	}
}

/// データベースの初期化
pub fn init() -> Result<(), sqlite::Error> {
	let mut db = db::Service::new();
	let connection = db.open();

	connection
		.execute("CREATE TABLE USERS(ID VARCHAR(999) NOT NULL, NAME VARCHAR(255) NOT NULL)")?;

	let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
	statement.bind(1, generate_uuid4().as_str())?;
	statement.bind(2, "John Lennon")?;
	statement.next()?;

	let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
	statement.bind(1, generate_uuid4().as_str())?;
	statement.bind(2, "Paul McCartney")?;
	statement.next()?;

	let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
	statement.bind(1, generate_uuid4().as_str())?;
	statement.bind(2, "Ringo Starr")?;
	statement.next()?;

	let mut statement = connection.prepare("INSERT INTO USERS(ID, NAME) VALUES(?, ?)")?;
	statement.bind(1, generate_uuid4().as_str())?;
	statement.bind(2, "George Harrison")?;
	statement.next()?;

	return Ok(());
}
