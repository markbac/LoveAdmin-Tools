mod data_structures;
mod database;
mod transform;
mod query;
mod config_loader;

use anyhow::{Result, Context}; 
use data_structures::{Wholegame, LoveAdmin};
use database::{setup_database, create_table, insert_loveadmin, insert_wholegame};


fn main() -> Result<()> {
    let conn = setup_database(None).context("Failed to setup database")?;
    
    //let conn = setup_database(Some("test_datbase.db"))?;

    // SQL to create the 'loveAdmin' table
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


    // SQL to create the 'wholegame' table
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

    let config = config_loader::load_config("config.yaml").expect("Failed to load configuration");
     

    // Create the 'loveadmin' table
    create_table(&conn, loveadmin_table_sql)?;
    create_table(&conn, wholegame_table_sql)?;

    // Example data to insert into 'loveadmin'
    // Initialize an empty or default instance of LoveAdmin
    let mut example_invoice = LoveAdmin::new();

    // Now, use setters to update the instance with the example data
    example_invoice.set_name("Company B".to_string());
    example_invoice.set_account_owner("Owner B".to_string());
    example_invoice.set_product("Product B".to_string());
    example_invoice.set_date("2023-04-03".to_string());
    example_invoice.set_invoiced(200.0);
    example_invoice.set_paid(150.0);
    example_invoice.set_pending(50.0);
    example_invoice.set_outstanding(50.0);
    example_invoice.set_failed(0);
    example_invoice.set_days_overdue(0);
    example_invoice.set_last_reminder_sent("2023-04-04".to_string());


    // Insert the example loveadmin data
    insert_loveadmin(&conn, &LoveAdmin::new()).context("Failed to insert loveadmin data")?;

    
    // Example data to insert into 'wholegame'
    // Initialize an instance of Wholegame with default values
    let mut example_wholegame = Wholegame::new();

    // Use setters to update the instance with the example data
    example_wholegame.set_first_names("Jane".to_string());
    example_wholegame.set_surname("Doe".to_string());
    example_wholegame.set_fan_id("987654321".to_string());
    example_wholegame.set_date_of_birth("2005-09-04".to_string());
    example_wholegame.set_age_group("U15".to_string());
    example_wholegame.set_gender("Female".to_string());
    example_wholegame.set_suspended(false);
    example_wholegame.set_team("City Juniors".to_string());
    example_wholegame.set_date_submitted("2023-09-01".to_string());
    example_wholegame.set_date_registered(Some("2023-09-02".to_string()));
    example_wholegame.set_registration_expiry(Some("2024-09-01".to_string()));
    example_wholegame.set_registration_status("Active".to_string());
    example_wholegame.set_email_address("janedoe@example.com".to_string());
    example_wholegame.set_parent_carer_name(Some("John Doe".to_string()));
    example_wholegame.set_parent_carer_email_address(Some("johndoe@example.com".to_string()));
    example_wholegame.set_emergency_contact(Some("John Doe".to_string()));
    example_wholegame.set_emergency_contact_phone_number(Some("555-1234".to_string()));
    example_wholegame.set_other_clubs(None);
    example_wholegame.set_consent_given(true);
    example_wholegame.set_contract_status("Registered".to_string());
    example_wholegame.set_photo_uploaded_date(Some("2023-09-01".to_string()));
    // Insert the example wholegame data
    insert_wholegame(&conn, &example_wholegame)?;

    // Apply transformations
    for transformation in &config.transformations {
        // Assuming you have a way to fetch or iterate over records
        let mut wholegame = data_structures::Wholegame::new();
        transform::apply_transformations(&mut wholegame, &transformation.rule);
        // Save or update records as necessary
    }

    // Execute queries
    for _query in &config.queries {
        query::execute_query(&conn, "SELECT * FROM your_table", "output.csv").context("Failed to execute query")?;
    }

    Ok(())
}
