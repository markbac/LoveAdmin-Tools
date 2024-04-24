// Define a struct to hold player data
pub struct Wholegame {
    first_names: String,
    surname: String,
    full_name: String,
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

pub struct LoveAdmin {
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

impl Wholegame {

    // Constructor method for Wholegame
    pub fn new() -> Self {
        Wholegame {
            first_names: "".to_string(),
            surname: "".to_string(),
            full_name: "".to_string(),
            fan_id: "".to_string(),
            date_of_birth: "".to_string(),
            age_group: "".to_string(),
            gender: "".to_string(),
            suspended: false,
            team: "".to_string(),
            date_submitted: "".to_string(),
            date_registered: None,
            registration_expiry: None,
            registration_status: "".to_string(),
            email_address: "".to_string(),
            parent_carer_name: None,
            parent_carer_email_address: None,
            emergency_contact: None,
            emergency_contact_phone_number: None,
            other_clubs: None,
            consent_given: false,
            contract_status: "".to_string(),
            photo_uploaded_date: None,
        }
    }
    
    // Getters
    pub fn get_first_names(&self) -> &String { &self.first_names }
    pub fn get_surname(&self) -> &String { &self.surname }
    pub fn get_fan_id(&self) -> &String { &self.fan_id }
    pub fn get_date_of_birth(&self) -> &String { &self.date_of_birth }
    pub fn get_age_group(&self) -> &String { &self.age_group }
    pub fn get_gender(&self) -> &String { &self.gender }
    pub fn is_suspended(&self) -> bool { self.suspended }
    pub fn get_team(&self) -> &String { &self.team }
    pub fn get_date_submitted(&self) -> &String { &self.date_submitted }
    pub fn get_date_registered(&self) -> &Option<String> { &self.date_registered }
    pub fn get_registration_expiry(&self) -> &Option<String> { &self.registration_expiry }
    pub fn get_registration_status(&self) -> &String { &self.registration_status }
    pub fn get_email_address(&self) -> &String { &self.email_address }
    pub fn get_parent_carer_name(&self) -> &Option<String> { &self.parent_carer_name }
    pub fn get_parent_carer_email_address(&self) -> &Option<String> { &self.parent_carer_email_address }
    pub fn get_emergency_contact(&self) -> &Option<String> { &self.emergency_contact }
    pub fn get_emergency_contact_phone_number(&self) -> &Option<String> { &self.emergency_contact_phone_number }
    pub fn get_other_clubs(&self) -> &Option<String> { &self.other_clubs }
    pub fn is_consent_given(&self) -> bool { self.consent_given }
    pub fn get_contract_status(&self) -> &String { &self.contract_status }
    pub fn get_photo_uploaded_date(&self) -> &Option<String> { &self.photo_uploaded_date }

    // Setters
    pub fn set_first_names(&mut self, value: String) { self.first_names = value; }
    pub fn set_surname(&mut self, value: String) { self.surname = value; }
    pub fn set_full_name(&mut self, value: String) { self.full_name = value; }
    pub fn set_fan_id(&mut self, value: String) { self.fan_id = value; }
    pub fn set_date_of_birth(&mut self, value: String) { self.date_of_birth = value; }
    pub fn set_age_group(&mut self, value: String) { self.age_group = value; }
    pub fn set_gender(&mut self, value: String) { self.gender = value; }
    pub fn set_suspended(&mut self, value: bool) { self.suspended = value; }
    pub fn set_team(&mut self, value: String) { self.team = value; }
    pub fn set_date_submitted(&mut self, value: String) { self.date_submitted = value; }
    pub fn set_date_registered(&mut self, value: Option<String>) { self.date_registered = value; }
    pub fn set_registration_expiry(&mut self, value: Option<String>) { self.registration_expiry = value; }
    pub fn set_registration_status(&mut self, value: String) { self.registration_status = value; }
    pub fn set_email_address(&mut self, value: String) { self.email_address = value; }
    pub fn set_parent_carer_name(&mut self, value: Option<String>) { self.parent_carer_name = value; }
    pub fn set_parent_carer_email_address(&mut self, value: Option<String>) { self.parent_carer_email_address = value; }
    pub fn set_emergency_contact(&mut self, value: Option<String>) { self.emergency_contact = value; }
    pub fn set_emergency_contact_phone_number(&mut self, value: Option<String>) { self.emergency_contact_phone_number = value; }
    pub fn set_other_clubs(&mut self, value: Option<String>) { self.other_clubs = value; }
    pub fn set_consent_given(&mut self, value: bool) { self.consent_given = value; }
    pub fn set_contract_status(&mut self, value: String) { self.contract_status = value; }
    pub fn set_photo_uploaded_date(&mut self, value: Option<String>) { self.photo_uploaded_date = value; }
}

impl LoveAdmin {
    // Constructor method
    pub fn new() -> LoveAdmin {
        LoveAdmin {
            name: "".to_string(),
            account_owner: "".to_string(),
            product: "".to_string(),
            date: "".to_string(),
            invoiced: 0.0,
            paid: 0.0,
            pending: 0.0,
            outstanding: 0.0,
            failed: 0,
            days_overdue: 0,
            last_reminder_sent: "".to_string(),
        }
    }
    // Getters
    pub fn get_name(&self) -> &String { &self.name }
    pub fn get_account_owner(&self) -> &String { &self.account_owner }
    pub fn get_product(&self) -> &String { &self.product }
    pub fn get_date(&self) -> &String { &self.date }
    pub fn get_invoiced(&self) -> f64 { self.invoiced }
    pub fn get_paid(&self) -> f64 { self.paid }
    pub fn get_pending(&self) -> f64 { self.pending }
    pub fn get_outstanding(&self) -> f64 { self.outstanding }
    pub fn get_failed(&self) -> i32 { self.failed }
    pub fn get_days_overdue(&self) -> i32 { self.days_overdue }
    pub fn get_last_reminder_sent(&self) -> &String { &self.last_reminder_sent }

    // Setters
    pub fn set_name(&mut self, value: String) { self.name = value; }
    pub fn set_account_owner(&mut self, value: String) { self.account_owner = value; }
    pub fn set_product(&mut self, value: String) { self.product = value; }
    pub fn set_date(&mut self, value: String) { self.date = value; }
    pub fn set_invoiced(&mut self, value: f64) { self.invoiced = value; }
    pub fn set_paid(&mut self, value: f64) { self.paid = value; }
    pub fn set_pending(&mut self, value: f64) { self.pending = value; }
    pub fn set_outstanding(&mut self, value: f64) { self.outstanding = value; }
    pub fn set_failed(&mut self, value: i32) { self.failed = value; }
    pub fn set_days_overdue(&mut self, value: i32) { self.days_overdue = value; }
    pub fn set_last_reminder_sent(&mut self, value: String) { self.last_reminder_sent = value; }
}

#[cfg(test)]
mod wholegame_tests {
    use super::*;

    #[test]
    fn test_wholegame_new_sets_default_values() {
        let wg = Wholegame::new();
        assert_eq!(wg.get_first_names(), "");
        assert_eq!(wg.get_surname(), "");
        assert_eq!(wg.get_fan_id(), "");
        assert_eq!(wg.get_date_of_birth(), "");
        assert_eq!(wg.get_age_group(), "");
        assert_eq!(wg.get_gender(), "");
        assert!(!wg.is_suspended());
        assert_eq!(wg.get_team(), "");
        assert_eq!(wg.get_date_submitted(), "");
        assert!(wg.get_date_registered().is_none());
        assert!(wg.get_registration_expiry().is_none());
        assert_eq!(wg.get_registration_status(), "");
        assert_eq!(wg.get_email_address(), "");
        assert!(wg.get_parent_carer_name().is_none());
        assert!(wg.get_parent_carer_email_address().is_none());
        assert!(wg.get_emergency_contact().is_none());
        assert!(wg.get_emergency_contact_phone_number().is_none());
        assert!(wg.get_other_clubs().is_none());
        assert!(!wg.is_consent_given());
        assert_eq!(wg.get_contract_status(), "");
        assert!(wg.get_photo_uploaded_date().is_none());
    }

    #[test]
    fn test_wholegame_setters_and_getters() {
        let mut wg = Wholegame::new();
        wg.set_first_names("John".to_string());
        wg.set_surname("Doe".to_string());
        wg.set_fan_id("123456".to_string());
        wg.set_date_of_birth("2000-01-01".to_string());
        wg.set_age_group("Adult".to_string());
        wg.set_gender("Male".to_string());
        wg.set_suspended(true);
        wg.set_team("Local FC".to_string());
        wg.set_date_submitted("2023-04-01".to_string());
        wg.set_date_registered(Some("2023-04-02".to_string()));
        wg.set_registration_expiry(Some("2024-04-01".to_string()));
        wg.set_registration_status("Active".to_string());
        wg.set_email_address("johndoe@example.com".to_string());
        wg.set_parent_carer_name(Some("Jane Doe".to_string()));
        wg.set_parent_carer_email_address(Some("janedoe@example.com".to_string()));
        wg.set_emergency_contact(Some("Emergency Contact".to_string()));
        wg.set_emergency_contact_phone_number(Some("123-456-7890".to_string()));
        wg.set_other_clubs(Some("Other Clubs".to_string()));
        wg.set_consent_given(true);
        wg.set_contract_status("Signed".to_string());
        wg.set_photo_uploaded_date(Some("2023-04-01".to_string()));

        assert_eq!(wg.get_first_names(), "John");
        assert_eq!(wg.get_surname(), "Doe");
        assert_eq!(wg.get_fan_id(), "123456");
        assert_eq!(wg.get_date_of_birth(), "2000-01-01");
        assert_eq!(wg.get_age_group(), "Adult");
        assert_eq!(wg.get_gender(), "Male");
        assert!(wg.is_suspended());
        assert_eq!(wg.get_team(), "Local FC");
        assert_eq!(wg.get_date_submitted(), "2023-04-01");
        assert_eq!(wg.get_date_registered().as_deref(), Some("2023-04-02"));
        assert_eq!(wg.get_registration_expiry().as_deref(), Some("2024-04-01"));
        assert_eq!(wg.get_registration_status(), "Active");
        assert_eq!(wg.get_email_address(), "johndoe@example.com");
        assert_eq!(wg.get_parent_carer_name().as_deref(), Some("Jane Doe"));
        assert_eq!(wg.get_parent_carer_email_address().as_deref(), Some("janedoe@example.com"));
        assert_eq!(wg.get_emergency_contact().as_deref(), Some("Emergency Contact"));
        assert_eq!(wg.get_emergency_contact_phone_number().as_deref(), Some("123-456-7890"));
        assert_eq!(wg.get_other_clubs().as_deref(), Some("Other Clubs"));
        assert!(wg.is_consent_given());
        assert_eq!(wg.get_contract_status(), "Signed");
        assert_eq!(wg.get_photo_uploaded_date().as_deref(), Some("2023-04-01"));
    }
}
#[cfg(test)]
mod loveadmin_tests {
    use super::*;

    #[test]
    fn test_loveadmin_new_sets_default_values() {
        let la = LoveAdmin::new();
        assert_eq!(la.get_name(), "");
        assert_eq!(la.get_account_owner(), "");
        assert_eq!(la.get_product(), "");
        assert_eq!(la.get_date(), "");
        assert_eq!(la.get_invoiced(), 0.0);
        assert_eq!(la.get_paid(), 0.0);
        assert_eq!(la.get_pending(), 0.0);
        assert_eq!(la.get_outstanding(), 0.0);
        assert_eq!(la.get_failed(), 0);
        assert_eq!(la.get_days_overdue(), 0);
        assert_eq!(la.get_last_reminder_sent(), "");
    }

    #[test]
    fn test_loveadmin_setters_and_getters() {
        let mut la = LoveAdmin::new();
        la.set_name("Company A".to_string());
        la.set_account_owner("Owner A".to_string());
        la.set_product("Product A".to_string());
        la.set_date("2023-05-01".to_string());
        la.set_invoiced(500.0);
        la.set_paid(300.0);
        la.set_pending(200.0);
        la.set_outstanding(200.0);
        la.set_failed(1);
        la.set_days_overdue(30);
        la.set_last_reminder_sent("2023-05-02".to_string());

        assert_eq!(la.get_name(), "Company A");
        assert_eq!(la.get_account_owner(), "Owner A");
        assert_eq!(la.get_product(), "Product A");
        assert_eq!(la.get_date(), "2023-05-01");
        assert_eq!(la.get_invoiced(), 500.0);
        assert_eq!(la.get_paid(), 300.0);
        assert_eq!(la.get_pending(), 200.0);
        assert_eq!(la.get_outstanding(), 200.0);
        assert_eq!(la.get_failed(), 1);
        assert_eq!(la.get_days_overdue(), 30);
        assert_eq!(la.get_last_reminder_sent(), "2023-05-02");
    }
}
