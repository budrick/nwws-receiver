use rusqlite::{named_params, Connection, Statement};

pub struct Db<'a> {
    connection: Connection,
    statement: Statement<'a>
}

pub struct DbRecord {
    time: String,
    ttaa: String,
    message: String
}

impl<'a> Db<'a> {
    pub fn new() -> Db<'a> {
        let mut path = std::env::current_dir().unwrap();
        path.push("bulletins");
        path.set_extension("sqlite");
    
        println!("Database path: {}", path.display());
        let connection = Connection::open(path).unwrap();
        let statement = connection.prepare("INSERT INTO bulletins VALUES (:time, :type, :text)").unwrap();
        Db { connection, statement}
    }

    pub fn init(self) {
        self.connection.execute("CREATE TABLE IF NOT EXISTS bulletins (time_rfc3339 text, type text, bulletin text)", []).unwrap();
        self.connection.execute("CREATE INDEX IF NOT EXISTS time_idx ON bulletins (time_rfc3339)", []).unwrap();
        self.connection.execute("CREATE INDEX IF NOT EXISTS type_ids ON bulletins (type)", []).unwrap();
        self.connection.execute("CREATE INDEX IF NOT EXISTS time_type ON bulletins (time_rfc3339, type)", []).unwrap();
            
    }

    pub fn write(self, record: DbRecord) {
        let _res = self.statement.execute(named_params!{":time": record.time, ":type": record.ttaa, ":text": record.message }).unwrap();

    }
}

pub fn create() -> Connection {
    let mut path = std::env::current_dir().unwrap();
    path.push("bulletins");
    path.set_extension("sqlite");

    println!("Database path: {}", path.display());
    return Connection::open(path).unwrap();
}

pub fn init(dbconn: &Connection) -> () {
    dbconn.execute("CREATE TABLE IF NOT EXISTS bulletins (time_rfc3339 text, type text, bulletin text)", []).unwrap();
    dbconn.execute("CREATE INDEX IF NOT EXISTS time_idx ON bulletins (time_rfc3339)", []).unwrap();
    dbconn.execute("CREATE INDEX IF NOT EXISTS type_ids ON bulletins (type)", []).unwrap();
    dbconn.execute("CREATE INDEX IF NOT EXISTS time_type ON bulletins (time_rfc3339, type)", []).unwrap();
}

pub fn stmt(dbconn: &Connection) -> Statement {
    let mut stmt = dbconn.prepare("INSERT INTO bulletins VALUES (:time, :type, :text)").unwrap();
    return stmt;
}

