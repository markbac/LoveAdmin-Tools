use rusqlite::{params, Connection, Result};
use crate::data_structures::{Wholegame, LoveAdmin}; // Adjust path as necessary


pub fn setup_database(explicit_path: Option<&str>) -> Result<Connection> {
    let db_path = match explicit_path {
        Some(path) => path,
        None => {
            #[cfg(debug_assertions)]
            {
                "debug_database.db"
            }
            #[cfg(not(debug_assertions))]
            {
                ":memory:"
            }
        },
    };

    Connection::open(db_path)
}

pub fn create_table(conn: &Connection, sql: &str) -> Result<()> {
    conn.execute(sql, [])?;
    Ok(())
}

pub fn insert_loveadmin(conn: &Connection, loveadmin: &LoveAdmin) -> Result<usize> {
    let insert_sql = "
        INSERT INTO loveadmin (
            Name, AccountOwner, Product, Date, Invoiced, Paid, Pending, Outstanding, Failed, DaysOverdue, LastReminderSent
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)";

    conn.execute(insert_sql, params![
        loveadmin.get_name(), loveadmin.get_account_owner(), loveadmin.get_product(), loveadmin.get_date(), 
        loveadmin.get_invoiced(), loveadmin.get_paid(), loveadmin.get_pending(), loveadmin.get_outstanding(), 
        loveadmin.get_failed(), loveadmin.get_days_overdue(), loveadmin.get_last_reminder_sent()
    ])
}


pub fn insert_wholegame(conn: &Connection, wholegame: &Wholegame) -> Result<usize> {
    let insert_sql = "
        INSERT INTO wholegame (
            FirstNames, Surname, FAN_ID, DateOfBirth, AgeGroup, Gender, Suspended,
            Team, DateSubmitted, DateRegistered, RegistrationExpiry, RegistrationStatus,
            EmailAddress, ParentCarerName, ParentCarerEmailAddress, EmergencyContact,
            EmergencyContactPhoneNumber, OtherClubs, ConsentGiven, ContractStatus, PhotoUploadedDate
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, 
                  ?16, ?17, ?18, ?19, ?20, ?21)";
    
    conn.execute(insert_sql, params![
        wholegame.get_first_names(), wholegame.get_surname(), wholegame.get_fan_id(), wholegame.get_date_of_birth(),
        wholegame.get_age_group(), wholegame.get_gender(), wholegame.is_suspended(), wholegame.get_team(),
        wholegame.get_date_submitted(), wholegame.get_date_registered(), wholegame.get_registration_expiry(),
        wholegame.get_registration_status(), wholegame.get_email_address(), wholegame.get_parent_carer_name(),
        wholegame.get_parent_carer_email_address(), wholegame.get_emergency_contact(),
        wholegame.get_emergency_contact_phone_number(), wholegame.get_other_clubs(), wholegame.is_consent_given(),
        wholegame.get_contract_status(), wholegame.get_photo_uploaded_date()
    ])
}



#[cfg(test)]
mod database_tests {
    use super::*;
    use std::fs;
    use uuid::Uuid;

    // Test with in-memory database
    #[test]
    fn test_in_memory_database_and_table_creation() -> Result<()> {
        let conn = setup_database(Some(":memory:"))?;
        create_test_table(&conn)?;
        assert_table_exists(&conn, "test_table")
    }

    // Test with a file-based database in debug mode
    #[cfg(debug_assertions)]
    #[test]
    fn test_file_based_database_and_table_creation_debug() -> Result<()> {
        let db_file_path = "temp_test_db_debug.db";
        let conn = setup_database(Some(db_file_path))?;
        create_test_table(&conn)?;
        let result = assert_table_exists(&conn, "test_table");
        fs::remove_file(db_file_path).expect("Failed to delete test database file.");
        result
    }

    // Function to create a test table
    fn create_test_table(conn: &Connection) -> Result<()> {
        let table_sql = "
            CREATE TABLE IF NOT EXISTS test_table (
                id INTEGER PRIMARY KEY,
                data TEXT NOT NULL
            )";
        conn.execute(table_sql, []).map(|_| ())
    }

    // Function to assert that a table exists
    fn assert_table_exists(conn: &Connection, table_name: &str) -> Result<()> {
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
        let tables: Vec<String> = stmt.query_map([table_name], |row| row.get(0))?.collect::<Result<Vec<String>, _>>()?;
        assert!(tables.contains(&table_name.to_string()));
        Ok(())
    }

    #[test]
    fn test_file_based_database_and_table_creation() -> Result<()> {
        let db_file_path = "test_db_file.sqlite";
        let conn = setup_database(Some(db_file_path))?;

        {
            // Limit the scope of `stmt` so it gets dropped before `conn`
            let table_sql = "
                CREATE TABLE IF NOT EXISTS test_table (
                    id INTEGER PRIMARY KEY,
                    data TEXT NOT NULL
                )";
            create_table(&conn, table_sql)?;

            let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='test_table'")?;
            let tables: Vec<String> = stmt.query_map([], |row| row.get(0))?.collect::<Result<Vec<String>, _>>()?;
            assert!(tables.contains(&"test_table".to_string()));
        } // `stmt` is dropped here because its scope ends

        // It's now safe to drop `conn` since `stmt` is no longer borrowing it
        drop(conn); // Explicitly dropping `conn` is actually unnecessary here since it will be automatically dropped at the end of the scope

        // Clean up: Remove the test database file after the test
        fs::remove_file(db_file_path).expect("Failed to delete test database file.");

        Ok(())
    }

    #[test]
    fn test_insert_loveadmin_data() -> Result<()> {
        let conn = setup_database(None)?;
        let loveadmin_table_sql = "
            CREATE TABLE IF NOT EXISTS loveadmin (
                id INTEGER PRIMARY KEY,
                Name TEXT NOT NULL COLLATE NOCASE,
                AccountOwner TEXT NOT NULL COLLATE NOCASE,
                Product TEXT NOT NULL COLLATE NOCASE,
                Date TEXT NOT NULL,
                Invoiced REAL NOT NULL,
                Paid REAL NOT NULL,
                Pending REAL NOT NULL,
                Outstanding REAL NOT NULL,
                Failed INTEGER NOT NULL,
                DaysOverdue INTEGER NOT NULL,
                LastReminderSent TEXT NOT NULL COLLATE NOCASE
            )";
    
        create_table(&conn, loveadmin_table_sql)?;

        // Adjusted to use the LoveAdmin struct
        let mut example_loveadmin_data = LoveAdmin::new();

        example_loveadmin_data.set_name("Test Company".to_string());
        example_loveadmin_data.set_account_owner("Test Owner".to_string());
        example_loveadmin_data.set_product("Test Product".to_string());
        example_loveadmin_data.set_date("2023-01-01".to_string());
        example_loveadmin_data.set_invoiced(100.0);
        example_loveadmin_data.set_paid(100.0);
        example_loveadmin_data.set_pending(0.0);
        example_loveadmin_data.set_outstanding(0.0);
        example_loveadmin_data.set_failed(0);
        example_loveadmin_data.set_days_overdue(0);
        example_loveadmin_data.set_last_reminder_sent("2023-01-02".to_string());
        
        // Call insert_loveadmin with a LoveAdmin instance
        insert_loveadmin(&conn, &example_loveadmin_data)?;

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM loveadmin WHERE Name = ?1")?;
        let count: i64 = stmt.query_row(params!["Test Company"], |row| row.get(0))?;
        assert!(count >= 1, "Expected count to be at least 1, got {}", count);
        Ok(())
    }

    #[test]
    fn test_insert_wholegame_data() -> Result<()> {
        let conn = setup_database(None)?;
        let wholegame_table_sql = "
            CREATE TABLE IF NOT EXISTS wholegame (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                FirstNames TEXT NOT NULL COLLATE NOCASE,
                Surname TEXT NOT NULL COLLATE NOCASE,
                FAN_ID TEXT UNIQUE NOT NULL COLLATE NOCASE,
                DateOfBirth DATE NOT NULL,
                AgeGroup TEXT NOT NULL COLLATE NOCASE,
                Gender TEXT NOT NULL COLLATE NOCASE,
                Suspended BOOLEAN NOT NULL,
                Team TEXT NOT NULL COLLATE NOCASE,
                DateSubmitted DATETIME NOT NULL,
                DateRegistered DATETIME,
                RegistrationExpiry DATE,
                RegistrationStatus TEXT NOT NULL COLLATE NOCASE,
                EmailAddress TEXT NOT NULL COLLATE NOCASE,
                ParentCarerName TEXT COLLATE NOCASE,
                ParentCarerEmailAddress TEXT COLLATE NOCASE,
                EmergencyContact TEXT COLLATE NOCASE,
                EmergencyContactPhoneNumber TEXT COLLATE NOCASE,
                OtherClubs TEXT COLLATE NOCASE,
                ConsentGiven BOOLEAN NOT NULL,
                ContractStatus TEXT NOT NULL COLLATE NOCASE,
                PhotoUploadedDate DATETIME
            )";
    

        
        create_table(&conn, wholegame_table_sql)?;

        let fan_id = Uuid::new_v4().to_string();

        // Example player data
        let mut wholegame = Wholegame::new();

        wholegame.set_first_names("John".to_string());
        wholegame.set_surname("Doe".to_string());
        wholegame.set_fan_id(fan_id);
        wholegame.set_date_of_birth("2000-01-01".to_string());
        wholegame.set_age_group("Adult".to_string());
        wholegame.set_gender("Male".to_string());
        wholegame.set_suspended(false);
        wholegame.set_team("Local FC".to_string());
        wholegame.set_date_submitted("2023-04-01".to_string());
        wholegame.set_date_registered(Some("2023-04-02".to_string()));
        wholegame.set_registration_expiry(Some("2024-04-01".to_string()));
        wholegame.set_registration_status("Active".to_string());
        wholegame.set_email_address("johndoe@example.com".to_string());
        wholegame.set_parent_carer_name(None);
        wholegame.set_parent_carer_email_address(None);
        wholegame.set_emergency_contact(Some("Jane Doe".to_string()));
        wholegame.set_emergency_contact_phone_number(Some("123-456-7890".to_string()));
        wholegame.set_other_clubs(None);
        wholegame.set_consent_given(true);
        wholegame.set_contract_status("Signed".to_string());
        wholegame.set_photo_uploaded_date(Some("2023-04-01".to_string()));
        

        insert_wholegame(&conn, &wholegame)?;
        
        // Verify the insertion
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM wholegame WHERE FAN_ID = ?1")?;
        let count: i64 = stmt.query_row(params![wholegame.get_fan_id()], |row| row.get(0))?;

        assert!(count >= 1, "Expected at least 1 record with FAN_ID {}, found {}", wholegame.get_fan_id(), count);


        Ok(())
    }
}
