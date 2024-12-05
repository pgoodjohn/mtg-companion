use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, Row, Statement};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::configuration::Configuration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: Uuid,
    pub name: String,
    pub set_code: String,
    pub created_at_utc: DateTime<Utc>,
    pub updated_at_utc: DateTime<Utc>,
}

impl Card {
    pub fn new(name: String, set_code: String) -> Self {
        Card {
            id: Uuid::now_v7(),
            name: name,
            set_code: set_code,
            created_at_utc: Utc::now(),
            updated_at_utc: Utc::now(),
        }
    }

    fn is_stored(&self) -> bool {
        false
    }

    pub fn save(&self, connection: &Connection) -> Result<&Self, ()> {
        if self.is_stored() {
            return Ok(self);
        }

        connection.execute(
            "INSERT INTO cards (id, name, set_code, created_at_utc, updated_at_utc) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                &self.id.to_string(), 
                &self.name,
                &self.set_code,
                &self.created_at_utc.to_rfc3339(), 
                &self.updated_at_utc.to_rfc3339()],
        ).unwrap();

        Ok(self)
    }

    fn from_row(row: &Row, connection: &Connection) -> Result<Self> {
        let uuid_string: String = row.get("id").unwrap();
        let created_at_string: String = row.get("created_at_utc").unwrap();
        let updated_at_string: String = row.get("updated_at_utc").unwrap();

        Ok(Card {
            id: Uuid::parse_str(&uuid_string).unwrap(),
            name: row.get("name").unwrap(),
            set_code: row.get("set_code").unwrap(),
            created_at_utc: DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&created_at_string).unwrap()),
            updated_at_utc: DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&&updated_at_string).unwrap())
        })
    }
}

#[tauri::command]
pub fn save_card_command(
    name: String,
    set_code: String,
    db: State<Pool<SqliteConnectionManager>>,
    configuration: State<Configuration>
) -> Result<String, String> {
    log::debug!("Running save card command for card: {} - {}", name, set_code);
    let card = Card::new(
        name,
        set_code
    );

    card.save(&db.get().unwrap()).unwrap();

    Ok(serde_json::to_string(&card).unwrap())
}

#[tauri::command]
pub fn load_cards_command(
    db: State<Pool<SqliteConnectionManager>>,
    configuration: State<Configuration>,
) -> Result<String, String> {

    log::debug!("Running lod cards command");
    let conn = db.get().unwrap(); // Get a connection from the pool
    let mut stmt = conn.prepare("SELECT * FROM cards").unwrap(); // Prepare the SQL statement
    let card_iter = stmt.query_map([], |row| {
        Card::from_row(row, &conn) // Map each row to a Card object
    }).unwrap();

    let mut cards = Vec::new();
    for card in card_iter {
        cards.push(card.unwrap()); // Collect all cards into a vector
    }

    Ok(serde_json::to_string(&cards).unwrap())
}

#[tauri::command]
pub fn delete_card_command(
    card_id: String,
    db: State<Pool<SqliteConnectionManager>>,
    configuration: State<Configuration>,
) -> Result<String, String> {
    log::debug!("Running delete card command for card ID: {}", card_id);
    let conn = db.get().unwrap(); // Get a connection from the pool

    let uuid = Uuid::parse_str(&card_id).map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM cards WHERE id = ?1",
        rusqlite::params![&uuid.to_string()],
    ).map_err(|e| e.to_string())?;

    Ok(format!("Card with ID {} deleted successfully", card_id))
}