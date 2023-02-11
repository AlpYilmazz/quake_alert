use std::fmt::Display;

use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Address, Message,
    SmtpTransport, Transport,
};

use crate::{
    config::{AlertAccount, AlertRule, ProgramConfig},
    process::QuakeData,
    util::{earth_dist_km, interpolate},
};

pub fn matches_alert_rule(alert_rule: &AlertRule, quake_data: &QuakeData) -> bool {
    let quake_mag = quake_data.magnitude.get_main();

    let dist = earth_dist_km(alert_rule.self_coord, quake_data.coord);
    let min_magnitude = if dist <= alert_rule.search_radius_km_1 {
        alert_rule.min_magnitude_1
    } else if dist <= alert_rule.search_radius_km_2 {
        let factor = (dist - alert_rule.search_radius_km_1)
            / (alert_rule.search_radius_km_2 - alert_rule.search_radius_km_1);
        interpolate(
            alert_rule.min_magnitude_1,
            alert_rule.min_magnitude_2,
            factor as f32,
        )
    } else {
        // if alert_rule.search_radius_km_2 <= dist
        return false; // does not match
    };

    let is_significant = quake_mag.ge(&min_magnitude);

    is_significant
}

#[derive(Debug)]
pub struct AlertContent {
    pub date: String,
    pub time: String,
    pub location: String,
    pub magnitude: f32,
}

impl Display for AlertContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Date: {}\nTime: {}\nLocation: {}\nMagnitude: {}\n",
            self.date, self.time, self.location, self.magnitude
        )
    }
}

pub fn send_alert_mail(alert_account: &AlertAccount, alerts: &[AlertContent]) {
    // dbg!(alerts);
    const SUBJECT: &'static str = "YAKIN <TEST> DEPREM";

    let (user, domain) = alert_account.mail.split_once('@').unwrap();
    let alert_mailbox = Mailbox::new(None, Address::new(user, domain).unwrap());

    let creds = Credentials::new(alert_account.mail.clone(), alert_account.password.clone());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let mut content = String::from("--------------------\n");
    for alert in alerts {
        content += &format!("{}", alert);
        content += &"--------------------\n";
    }
    let content = content;
    println!("{}\n", &content);

    let email = Message::builder()
        .from(alert_mailbox.clone())
        .to(alert_mailbox.clone())
        .subject(SUBJECT)
        .body(content)
        .unwrap();

    if !ProgramConfig::get().run_mode.is_debug() {
        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => println!("Could not send email: {:?}", e),
        }
    }
}
