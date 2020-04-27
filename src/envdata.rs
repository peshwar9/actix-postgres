use serde::{Deserialize};
use dotenv::dotenv;

#[derive(Clone, Deserialize,Debug)]
pub struct Envdata {
    pub database_url: String,
    pub cargo_pkg_version: String,
    pub cors_origin: String,
    pub server: String,
}

lazy_static! {
    pub static ref ENVDATA: Envdata = get_envdata();
}

fn get_envdata() -> Envdata {
    dotenv().ok();

    match envy::from_env::<Envdata>() {
        Ok(envdata) => envdata,
        Err(err) => panic!("Environment variables not set properly. Aborting") 
    }
}
