use std::{env, path::Path};

use axum::{extract::Query, http::header, response::IntoResponse};
use wallet_pass::{
    template::{Barcode, BarcodeFormat, Details, Field},
    Pass,
};

use crate::models::Pass as PassModel;

pub async fn passes_handler(Query(data): Query<PassModel>) -> Result<impl IntoResponse, String> {
    match (data.validate_date(), data.validate_time()) {
        (Ok(_), Ok(_)) => format!(
            "Your pass for {} at {} on {} at {} has been confirmed.",
            data.name, data.location, data.date, data.time
        ),
        (Err(_), Ok(_)) => "Invalid date format. Please use yyyy-mm-dd.".to_string(),
        (Ok(_), Err(_)) => "Invalid time format. Please use HH:MM.".to_string(),
        (Err(_), Err(_)) => {
            "Invalid date and time format. Please use yyyy-mm-dd for date and HH:MM for time."
                .to_string()
        }
    };

    println!(
        "-- Loading template from {}",
        env::var("TEMPLATE_PATH").unwrap()
    );
    let mut pass = Pass::from_path(Path::new(env::var("TEMPLATE_PATH").unwrap().as_str())).unwrap();

    // Set general attributes
    pass.pass_type_identifier(env::var("APPLE_PASS_TYPE_IDENTIFIER").unwrap().as_str());
    pass.team_identifier(env::var("APPLE_TEAM_IDENTIFIER").unwrap().as_str());

    // Set user specific attributes
    pass.serial_number(data.code.as_str());

    pass.barcode(Barcode::new(
        BarcodeFormat::PkBarcodeFormatQr,
        data.code.as_str(),
        "iso-8859-1",
    ));

    let mut event_ticket = Details::new();

    let mut field = Field::new_string("event", data.name.as_str());
    field.label("Event");
    event_ticket.add_primary_field(field);

    let mut field = Field::new_string("date", data.date.as_str());
    field.label("Date");
    event_ticket.add_secondary_field(field);

    let mut field = Field::new_string("time", data.time.as_str());
    field.label("Time");
    event_ticket.add_secondary_field(field);

    let mut field = Field::new_string("loc", data.location.as_str());
    field.label("Location");
    event_ticket.add_auxiliary_field(field);

    pass.event_ticket(event_ticket);

    println!(
        "-- Loading certificate from {}",
        env::var("CERTIFICATE_PATH").unwrap()
    );
    // Sign, comprass and save pass
    pass.export_to_file(
        env::var("CERTIFICATE_PATH").unwrap().as_str(),
        env::var("CERTIFICATE_PASSWORD").unwrap().as_str(),
        Path::new("keys/apple_wdrca.pem"),
        Path::new("./Eventpass.pkpass"),
    )
    .unwrap();

    // Get content of file as generic bytes
    let pass_content = std::fs::read("./Eventpass.pkpass").unwrap();

    // Return content of file as response
    Ok((
        [
            (header::CONTENT_TYPE, "application/vnd.apple.pkpass"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"Eventpass.pkpass\"",
            ),
        ],
        pass_content,
    ))
}
