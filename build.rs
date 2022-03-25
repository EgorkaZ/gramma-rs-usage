use std::error::Error;

extern crate gramma_rs;

fn main() -> Result<(), Box<dyn Error>>
{
    gramma_rs::codegen::process_grs_files()
}