use crate::{compilation, config, error, problem, testing};

pub mod connection_manager;
mod credentials;
mod unzip;
pub mod resource;

pub use credentials::Credentials;
use crate::fetch::resource::Resource;

pub fn fetch_resources(
    problem: &problem::Problem,
    config: &config::Config,
) -> Result<(bool, bool), crate::Error> {
    let connection =
        connection_manager::ConnectionManager::new(config).map_err(|e| crate::Error {
            description: format!("Couldn't start the connection manager: {}", e),
            exitcode: exitcode::IOERR,
        })?;

    let mut compilation_resources: Vec<Box<dyn Resource>> = Vec::new();
    compilation::P1XX.write_required_resources(problem, &connection, &mut compilation_resources);
    let mut testing_resources: Vec<Box<dyn Resource>> = Vec::new();
    testing::write_required_resources(problem, &connection, &mut testing_resources);

    let mut compilation = true;
    for resource in compilation_resources {
        if let Err(e) = resource.acquire() {
            error!("Compilation resource acquisition failed: {}", e);
            compilation = false;
        }
    }

    let mut testing = true;
    for resource in testing_resources {
        if let Err(e) = resource.acquire() {
            error!("Testing resource acquisition failed: {}", e);
            testing = false;
        }
    }

    Ok((compilation, testing))
}