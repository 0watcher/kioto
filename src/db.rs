use std::path::Path;

use polodb_core::{Collection, Database, Result as pdbResult};

use crate::schema::{LocalData, Message, Room};

pub struct DbRepo {
    pub rooms: Collection<Room>,
    pub messages: Collection<Message>,
    pub local_data: Collection<LocalData>,
    _db: Database,
}

impl DbRepo {
    pub fn init(filepath: &Path) -> pdbResult<Self> {
        let db = Database::open_file(filepath)?;

        Ok(DbRepo {
            rooms: db.collection::<Room>("rooms"),
            messages: db.collection::<Message>("messages"),
            local_data: db.collection::<LocalData>("local_data"),
            _db: db,
        })
    }

    pub(crate) fn memory_init() -> pdbResult<Self> {
        let db = Database::open_memory()?;

        Ok(DbRepo {
            rooms: db.collection::<Room>("rooms"),
            messages: db.collection::<Message>("messages"),
            local_data: db.collection::<LocalData>("local_data"),
            _db: db,
        })
    }
}
