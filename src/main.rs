use std::fs::File;

use polars::prelude::*;

mod basic;
mod extra;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        std::env::set_var("POLARS_FMT_MAX_ROWS", "30");
    }

    // read the parquet file here
    // print the schema
    let f = File::open("./data/yellow_tripdata_2024-01.parquet")?;
    let pr = ParquetReader::new(f);
    // println!("{:?}", pr.schema());

    let df = pr.finish()?;
    // println!("{:?}", df.schema());
    // println!("{}", df.head(Some(5)));

    // basic::drill_1_print_schemea(&df);
    // basic::drill_2_mean_trip_distance(&df)?;
    // basic::drill_3_avg_tip_per_payment_type(df)?;
    // basic::drill_4_avg_trip_distance_and_total_revenue_per_hour(df)?;
    // basic::drill_5_trip_distance_greater_than(df.lazy())?;
    // basic::drill_6_passenter_count_gt_3(df.lazy())?;
    // basic::drill_7_trips_where_type_is_card_and_tip_is_gt_0(df.lazy())?;
    // basic::drill_8_new_trip_percentage(df.lazy())?;
    // basic::drill_9(df.lazy())?;
    // basic::drill_10(df.lazy())?;
    // basic::drill_11(df.lazy())?;
    // basic::drill_12(df.lazy())?;
    // basic::drill_13(df.lazy())?;
    // basic::drill_14(df.lazy())?;

    // extra::drill_1(df.lazy())?;
    extra::drill_2(df.lazy())?;
    Ok(())
}
