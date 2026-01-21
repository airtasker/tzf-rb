use lazy_static::lazy_static;
use magnus::{function, prelude::*, Error, Ruby};

use tzf_rs::DefaultFinder;

lazy_static! {
  static ref FINDER: DefaultFinder = DefaultFinder::new();
}

fn get_tz_name(lat: f64, lng: f64) -> String {
  FINDER.get_tz_name(lng, lat).to_string()
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("TZF")?;

    module.define_singleton_method("tz_name", function!(get_tz_name, 2))?;

    Ok(())
}
