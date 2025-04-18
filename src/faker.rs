use fake::faker::address::en::*;
use fake::faker::color::raw::Color;
use fake::faker::internet::en::*;
use fake::faker::lorem::en::*;
use fake::faker::name::en::*;
use fake::Fake;
use std::collections::HashMap;
use fake::locales::EN;
use csv::Writer;


pub fn generate_fake_data_with_types(
    columns: Vec<(&str, &str)>,
    rows: usize,
) -> Vec<HashMap<String, String>> {
    let mut data = Vec::new();

    for _ in 0..rows {
        let mut row = HashMap::new();

        for (name, fake_type) in &columns {
            let value = match *fake_type {
                "FirstName" => FirstName().fake::<String>(),
                "LastName" => LastName().fake::<String>(),
                "CityName" => CityName().fake::<String>(),
                "StreetName" => StreetName().fake::<String>(),
                "ZipCode" => ZipCode().fake::<String>(),
                "Email" => FreeEmail().fake::<String>(),
                "Username" => Username().fake::<String>(),
                "Password" => Password(8..12).fake::<String>(),
                "Color" => Color(EN).fake::<String>(),
                "Paragraph" => Paragraph(1..3).fake::<String>(),
                "Number" => (1..100).fake::<u32>().to_string(),
                _ => panic!("Unsupported fake type: {}", fake_type),
            };

            row.insert(name.to_string(), value);
        }

        data.push(row);
    }

    data
}

pub fn write_data_to_csv(
    data: Vec<HashMap<String, String>>,
    file_path: &str,
) -> std::io::Result<()> {
    let mut wtr = Writer::from_path(file_path)?;

    // Define the column order based on the first row
    let headers: Vec<String> = if let Some(first_row) = data.first() {
        first_row.keys().cloned().collect()
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No data to write",
        ));
    };

    // Write the header row
    wtr.write_record(&headers)?;

    // Write each row in the same order as the headers
    for row in data {
        let values: Vec<String> = headers
            .iter()
            .map(|header| row.get(header).cloned().unwrap_or_default())
            .collect();
        wtr.write_record(values)?;
    }

    wtr.flush()?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_fake_data_with_types() {
        let columns = vec![
            ("FirstName", "FirstName"),
            ("LastName", "LastName"),
            ("CityName", "CityName"),
        ];

        let fake_data = generate_fake_data_with_types(columns, 5);

        assert_eq!(fake_data.len(), 5);
        assert_eq!(fake_data[0].len(), 3);
        assert!(fake_data[0].contains_key("FirstName"));
        assert!(fake_data[0].contains_key("LastName"));
        assert!(fake_data[0].contains_key("CityName"));
    }
}