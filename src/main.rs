use rusqlite::{params, Connection, Result};

// Define a struct to hold player data
struct Wholegame {
    first_names: String,
    surname: String,
    fan_id: String,
    date_of_birth: String, // Consider using a date type for real applications
    age_group: String,
    gender: String,
    suspended: bool,
    team: String,
    date_submitted: String, // Consider using a date type
    date_registered: Option<String>, // Optional fields are represented using Option
    registration_expiry: Option<String>,
    registration_status: String,
    email_address: String,
    parent_carer_name: Option<String>,
    parent_carer_email_address: Option<String>,
    emergency_contact: Option<String>,
    emergency_contact_phone_number: Option<String>,
    other_clubs: Option<String>,
    consent_given: bool,
    contract_status: String,
    photo_uploaded_date: Option<String>,
}

struct LoveAdmin {
    name: String,
    account_owner: String,
    product: String,
    date: String, // Consider using a date type for real applications
    invoiced: f64,
    paid: f64,
    pending: f64,
    outstanding: f64,
    failed: i32,
    days_overdue: i32,
    last_reminder_sent: String, // Consider using a date type
}


fn main() -> Result<()> {
    let conn = setup_database(None)?;
    
    //let conn = setup_database(Some("test_datbase.db"))?;

    // SQL to create the 'loveAdmin' table
    let loveadmin_table_sql = "
        CREATE TABLE IF NOT EXISTS loveadmin (
            id INTEGER PRIMARY KEY,
            Name TEXT NOT NULL,
            AccountOwner TEXT NOT NULL,
            Product TEXT NOT NULL,
            Date TEXT NOT NULL,
            Invoiced REAL NOT NULL,
            Paid REAL NOT NULL,
            Pending REAL NOT NULL,
            Outstanding REAL NOT NULL,
            Failed INTEGER NOT NULL,
            DaysOverdue INTEGER NOT NULL,
            LastReminderSent TEXT NOT NULL
        )";

    // SQL to create the 'wholegame' table
    let wholegame_table_sql = "
        CREATE TABLE IF NOT EXISTS wholegame (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            FirstNames TEXT NOT NULL,
            Surname TEXT NOT NULL,
            FAN_ID TEXT UNIQUE NOT NULL,
            DateOfBirth DATE NOT NULL,
            AgeGroup TEXT NOT NULL,
            Gender TEXT NOT NULL,
            Suspended BOOLEAN NOT NULL,
            Team TEXT NOT NULL,
            DateSubmitted DATETIME NOT NULL,
            DateRegistered DATETIME,
            RegistrationExpiry DATE,
            RegistrationStatus TEXT NOT NULL,
            EmailAddress TEXT NOT NULL,
            ParentCarerName TEXT,
            ParentCarerEmailAddress TEXT,
            EmergencyContact TEXT,
            EmergencyContactPhoneNumber TEXT,
            OtherClubs TEXT,
            ConsentGiven BOOLEAN NOT NULL,
            ContractStatus TEXT NOT NULL,
            PhotoUploadedDate DATETIME
        )";
         

    // Create the 'loveadmin' table
    create_table(&conn, loveadmin_table_sql)?;
    create_table(&conn, wholegame_table_sql)?;

    // Example data to insert into 'loveadmin'
    let example_invoice = LoveAdmin {
        name: "Company B".to_string(),
        account_owner: "Owner B".to_string(),
        product: "Product B".to_string(),
        date: "2023-04-03".to_string(),
        invoiced: 200.0,
        paid: 150.0,
        pending: 50.0,
        outstanding: 50.0,
        failed: 0,
        days_overdue: 0,
        last_reminder_sent: "2023-04-04".to_string(),
    };

    // Insert the example loveadmin data
    insert_loveadmin(&conn, &example_invoice)?;

    
    // Example data to insert into 'wholegame'
    let example_wholegame = Wholegame {
        first_names: "Jane".to_string(),
        surname: "Doe".to_string(),
        fan_id: "987654321".to_string(),
        date_of_birth: "2005-09-04".to_string(),
        age_group: "U15".to_string(),
        gender: "Female".to_string(),
        suspended: false,
        team: "City Juniors".to_string(),
        date_submitted: "2023-09-01".to_string(),
        date_registered: Some("2023-09-02".to_string()),
        registration_expiry: Some("2024-09-01".to_string()),
        registration_status: "Active".to_string(),
        email_address: "janedoe@example.com".to_string(),
        parent_carer_name: Some("John Doe".to_string()),
        parent_carer_email_address: Some("johndoe@example.com".to_string()),
        emergency_contact: Some("John Doe".to_string()),
        emergency_contact_phone_number: Some("555-1234".to_string()),
        other_clubs: None,
        consent_given: true,
        contract_status: "Registered".to_string(),
        photo_uploaded_date: Some("2023-09-01".to_string()),
    };
    
    // Insert the example wholegame data
    insert_wholegame(&conn, &example_wholegame)?;


    Ok(())
}

fn setup_database(db_path: Option<&str>) -> Result<Connection> {
    match db_path {
        Some(path) => Connection::open(path),
        None => Connection::open_in_memory(),
    }
}

fn create_table(conn: &Connection, sql: &str) -> Result<()> {
    conn.execute(sql, [])?;
    Ok(())
}

fn insert_loveadmin(conn: &Connection, loveadmin: &LoveAdmin) -> Result<usize> {
    let insert_sql = "
        INSERT INTO loveadmin (
            Name, AccountOwner, Product, Date, Invoiced, Paid, Pending, Outstanding, Failed, DaysOverdue, LastReminderSent
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)";

    conn.execute(insert_sql, params![
        loveadmin.name, loveadmin.account_owner, loveadmin.product, loveadmin.date, loveadmin.invoiced,
        loveadmin.paid, loveadmin.pending, loveadmin.outstanding, loveadmin.failed, loveadmin.days_overdue,
        loveadmin.last_reminder_sent
    ])
}


fn insert_wholegame(conn: &Connection, wholegame: &Wholegame) -> Result<usize> {
    let insert_sql = "
        INSERT INTO wholegame (
            FirstNames, Surname, FAN_ID, DateOfBirth, AgeGroup, Gender, Suspended,
            Team, DateSubmitted, DateRegistered, RegistrationExpiry, RegistrationStatus,
            EmailAddress, ParentCarerName, ParentCarerEmailAddress, EmergencyContact,
            EmergencyContactPhoneNumber, OtherClubs, ConsentGiven, ContractStatus, PhotoUploadedDate
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)";
    
    conn.execute(insert_sql, params![
        wholegame.first_names, wholegame.surname, wholegame.fan_id, wholegame.date_of_birth,
        wholegame.age_group, wholegame.gender, wholegame.suspended, wholegame.team,
        wholegame.date_submitted, wholegame.date_registered, wholegame.registration_expiry,
        wholegame.registration_status, wholegame.email_address, wholegame.parent_carer_name,
        wholegame.parent_carer_email_address, wholegame.emergency_contact,
        wholegame.emergency_contact_phone_number, wholegame.other_clubs, wholegame.consent_given,
        wholegame.contract_status, wholegame.photo_uploaded_date
    ])
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_in_memory_database_and_table_creation() -> Result<()> {
        let conn = setup_database(None)?;
        let table_sql = "
            CREATE TABLE IF NOT EXISTS test_table (
                id INTEGER PRIMARY KEY,
                data TEXT NOT NULL
            )";
        create_table(&conn, table_sql)?;

        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='test_table'")?;
        let tables: Vec<String> = stmt.query_map([], |row| row.get(0))?.collect::<Result<Vec<String>, _>>()?;
        assert!(tables.contains(&"test_table".to_string()));
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
                Name TEXT NOT NULL,
                AccountOwner TEXT NOT NULL,
                Product TEXT NOT NULL,
                Date TEXT NOT NULL,
                Invoiced REAL NOT NULL,
                Paid REAL NOT NULL,
                Pending REAL NOT NULL,
                Outstanding REAL NOT NULL,
                Failed INTEGER NOT NULL,
                DaysOverdue INTEGER NOT NULL,
                LastReminderSent TEXT NOT NULL
            )";
        create_table(&conn, loveadmin_table_sql)?;

        // Adjusted to use the LoveAdmin struct
        let example_loveadmin_data = LoveAdmin {
            name: "Test Company".to_string(),
            account_owner: "Test Owner".to_string(),
            product: "Test Product".to_string(),
            date: "2023-01-01".to_string(),
            invoiced: 100.0,
            paid: 100.0,
            pending: 0.0,
            outstanding: 0.0,
            failed: 0,
            days_overdue: 0,
            last_reminder_sent: "2023-01-02".to_string(),
        };

        // Call insert_loveadmin with a LoveAdmin instance
        insert_loveadmin(&conn, &example_loveadmin_data)?;

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM loveadmin WHERE Name = ?1")?;
        let count: i64 = stmt.query_row(params!["Test Company"], |row| row.get(0))?;
        assert_eq!(count, 1);
        Ok(())
    }

    #[test]
    fn test_insert_wholegame_data() -> Result<()> {
        let conn = setup_database(None)?;
        let wholegame_table_sql = "
            CREATE TABLE IF NOT EXISTS wholegame (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                FirstNames TEXT NOT NULL,
                Surname TEXT NOT NULL,
                FAN_ID TEXT UNIQUE NOT NULL,
                DateOfBirth DATE NOT NULL,
                AgeGroup TEXT NOT NULL,
                Gender TEXT NOT NULL,
                Suspended BOOLEAN NOT NULL,
                Team TEXT NOT NULL,
                DateSubmitted DATETIME NOT NULL,
                DateRegistered DATETIME,
                RegistrationExpiry DATE,
                RegistrationStatus TEXT NOT NULL,
                EmailAddress TEXT NOT NULL,
                ParentCarerName TEXT,
                ParentCarerEmailAddress TEXT,
                EmergencyContact TEXT,
                EmergencyContactPhoneNumber TEXT,
                OtherClubs TEXT,
                ConsentGiven BOOLEAN NOT NULL,
                ContractStatus TEXT NOT NULL,
                PhotoUploadedDate DATETIME
            )";

        
        create_table(&conn, wholegame_table_sql)?;

        // Example player data
        let wholegame = Wholegame {
            first_names: "John".to_string(),
            surname: "Doe".to_string(),
            fan_id: "123456".to_string(),
            date_of_birth: "2000-01-01".to_string(),
            age_group: "Adult".to_string(),
            gender: "Male".to_string(),
            suspended: false,
            team: "Local FC".to_string(),
            date_submitted: "2023-04-01".to_string(),
            date_registered: Some("2023-04-02".to_string()),
            registration_expiry: Some("2024-04-01".to_string()),
            registration_status: "Active".to_string(),
            email_address: "johndoe@example.com".to_string(),
            parent_carer_name: None,
            parent_carer_email_address: None,
            emergency_contact: Some("Jane Doe".to_string()),
            emergency_contact_phone_number: Some("123-456-7890".to_string()),
            other_clubs: None,
            consent_given: true,
            contract_status: "Signed".to_string(),
            photo_uploaded_date: Some("2023-04-01".to_string()),
        };

        insert_wholegame(&conn, &wholegame)?;
        
        // Verify the insertion
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM wholegame WHERE FAN_ID = ?1")?;
        let count: i64 = stmt.query_row(params![wholegame.fan_id], |row| row.get(0))?;
        assert_eq!(count, 1);

        Ok(())
    }
}
