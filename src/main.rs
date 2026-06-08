use std::fs::File;

use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the parquet file here
    // print the schema
    let f = File::open("./data/yellow_tripdata_2024-01.parquet")?;
    let mut pr = ParquetReader::new(f);
    println!("{:?}", pr.schema());
    Ok(())
}
