use std::fs::File;

use polars::prelude::*;

#[allow(unused)]
fn drill_1_print_schemea(df: &DataFrame) {
    println!("{:?}", df.schema());
}

#[allow(unused)]
fn drill_2_mean_trip_distance(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    // as_series() returns Option<&Series>
    // x.mean() returns Option<Float64>
    // without and_then this returns Option<Option<Float64>>
    // and_then is essentially flatmap
    let mean = df
        .column("trip_distance")?
        .as_series()
        .and_then(|x| x.mean());
    println!("{}", mean.unwrap_or(0.0));
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the parquet file here
    // print the schema
    let f = File::open("./data/yellow_tripdata_2024-01.parquet")?;
    let pr = ParquetReader::new(f);
    // println!("{:?}", pr.schema());

    let df = pr.finish()?;
    let _ = drill_2_mean_trip_distance(&df);
    // println!("{:?}", df.schema());
    // println!("{}", df.head(Some(5)));

    Ok(())
}
