use rocket_contrib::databases::redis;

#[database("COLOROO_DB")]
pub struct ColorsDbCon(redis::Connection);
