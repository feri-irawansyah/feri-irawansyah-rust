
use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, Utc, Timelike};

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

pub fn format_wib_date(date_str: &str) -> String {
    // Parse input sebagai UTC
    let utc_datetime = DateTime::parse_from_rfc3339(date_str)
        .unwrap_or_else(|_| Utc::now().into());

    // Konversi ke WIB (UTC+7)
    let wib_offset = FixedOffset::east_opt(7 * 3600).unwrap();
    let wib_datetime = utc_datetime.with_timezone(&wib_offset);

    // Format 12 jam
    // let hour = wib_datetime.hour();
    // let hour12 = if hour % 12 == 0 { 12 } else { hour % 12 };
    // let minutes = wib_datetime.minute();
    // let is_pm = hour >= 12;

    let weekday = wib_datetime.format("%a"); // Sen, Sel, Rab, dst
    let month = wib_datetime.format("%b");   // Jan, Feb, dst
    let day = wib_datetime.day();
    let year = wib_datetime.year();

    // format!(
    //     "{:02}:{:02} {} WIB, {} {} {}, {}",
    //     hour12,
    //     minutes,
    //     if is_pm { "PM" } else { "AM" },
    //     weekday,
    //     month,
    //     day,
    //     year
    // )
    format!(
        "{} {} {}, {}",
        weekday,
        month,
        day,
        year
    )
}