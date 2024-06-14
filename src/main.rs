use serde::Deserialize;
use std::collections::HashMap;
use rustlearn::prelude::*; use rustlearn::datasets::iris;
use rustlearn::svm::libsvm::svc::{Hyperparameters, KernelType};
use std::{
    error::Error,
    process,
};

#[derive(Debug, Deserialize)]
struct Thyriod{
    // TODO: fill struct
}   

fn run_model() -> Result<(), Box<dyn Error>>{
    let mut rdr = csv::Reader::from_path("Thyroid_Diff.csv")?;
    
    // TODO: train thyroid
    
    Ok(())
}

fn main() {
    if let Err(err) = run_model() {
        println!("{}", err);
        process::exit(1);
    }
}
