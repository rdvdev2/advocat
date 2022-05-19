mod diff_display;
mod test;
mod testsuite;

pub use testsuite::Error;
pub use testsuite::TestSuite;
use crate::fetch::connection_manager::ConnectionManager;
use crate::fetch::resource;
use crate::fetch::resource::{Resource};
use crate::problem;

pub fn write_required_resources<'a>(problem: &problem::Problem, connection: &'a ConnectionManager, resources: &mut Vec<Box<dyn Resource + 'a>>) {
    let zip = resource::OnlineResource::new(
        problem.work_dir.join("problem.zip"),
        problem.zip_url.clone(),
        connection
    );

    let tests = resource::UnzipResource::new(
        zip,
        problem.work_dir.join("samples")
    );
    resources.push(Box::new(tests));
}