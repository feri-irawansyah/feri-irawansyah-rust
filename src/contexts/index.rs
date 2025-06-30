
use chrono::{Datelike, NaiveDate, Utc};

pub fn hitung_usia(tanggal_lahir: &str) -> Option<i32> {
    // Parse tanggal lahir, formatnya "YYYY-MM-DD"
    let birth_date = NaiveDate::parse_from_str(tanggal_lahir, "%Y-%m-%d").ok()?;
    let today = Utc::now().date_naive();

    let mut usia = today.year() - birth_date.year();

    // Jika ulang tahun belum lewat, kurangi 1
    if (today.month(), today.day()) < (birth_date.month(), birth_date.day()) {
        usia -= 1;
    }

    Some(usia)
}