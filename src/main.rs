use std::fs::File;

use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the parquet file here
    // print the schema
    let f = File::open("./data/yellow_tripdata_2024-01.parquet")?;
    let pr = ParquetReader::new(f);
    // println!("{:?}", pr.schema());

    let df = pr.finish()?;
    println!("{}", df.head(Some(5)));
    Ok(())
}
