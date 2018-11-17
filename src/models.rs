#[derive(Queryable, Debug)]
pub struct Measurement {
    pub id: u64,
    pub temperature: f64,
    pub humidity: f64,
    pub comment: String,
}
