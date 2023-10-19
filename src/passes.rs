use std::io::Cursor;
use std::{env, path::Path};

use axum::{extract::Query, http::header, response::IntoResponse};
use wallet_pass::{
    template::{Barcode, BarcodeFormat, Details, Field},
    Pass,
};

use crate::models::Pass as PassModel;

pub async fn passes_handler(Query(data): Query<PassModel>) -> Result<impl IntoResponse, String> {
    let error = match (data.validate_date(), data.validate_time()) {
        (Ok(_), Ok(_)) => "".to_string(),
        (Err(_), Ok(_)) => "Invalid date format. Please use yyyy-mm-dd.".to_string(),
        (Ok(_), Err(_)) => "Invalid time format. Please use HH:MM.".to_string(),
        (Err(_), Err(_)) => {
            "Invalid date and time format. Please use yyyy-mm-dd for date and HH:MM for time."
                .to_string()
        }
    };

    if !error.is_empty() {
        return Err(error);
    }

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

    // Set relevant date
    let date = format!("{}T{}:00+02:00", data.date, data.time);
    pass.relevant_date(date.as_str());

    let mut event_ticket = Details::new();

    let mut field = Field::new_string("event", data.title.as_str());
    field.label("Event");
    event_ticket.add_primary_field(field);

    let mut field = Field::new_string("date", data.date.as_str());
    field.label("Date");
    event_ticket.add_secondary_field(field);

    let mut field = Field::new_string("time", data.time.as_str());
    field.label("Time");
    event_ticket.add_secondary_field(field);

    if let Some(name) = data.name {
        if !name.is_empty() {
            let mut field = Field::new_string("name", name.as_str());
            field.label("Name");
            event_ticket.add_auxiliary_field(field);
        }
    }

    if let Some(location) = data.location {
        if !location.is_empty() {
            let mut field = Field::new_string("loc", location.as_str());
            field.label("Location");
            event_ticket.add_auxiliary_field(field);
        }
    }

    pass.event_ticket(event_ticket);

    println!(
        "-- Loading certificate from {}",
        env::var("CERTIFICATE_PATH").unwrap()
    );

    // Sign, compress and save pass
    let pass_cursor = pass
        .export(
            env::var("CERTIFICATE_PATH").unwrap().as_str(),
            env::var("CERTIFICATE_PASSWORD").unwrap().as_str(),
            Path::new("keys/apple_wdrca.pem"),
            Cursor::new(Vec::<u8>::new()),
        )
        .unwrap();

    // Return content of file as response
    Ok((
        [
            (header::CONTENT_TYPE, "application/vnd.apple.pkpass"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"Eventpass.pkpass\"",
            ),
        ],
        pass_cursor.into_inner(),
    ))
}
