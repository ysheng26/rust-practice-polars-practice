// use std::fs::File;

use polars::prelude::*;

mod basic;
mod extra;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        std::env::set_var("POLARS_FMT_MAX_ROWS", "30");
    }

    // read the parquet file here
    // print the schema
    // let f = File::open("./data/yellow_tripdata_2024-01.parquet")?;
    // let pr = ParquetReader::new(f);
    // println!("{:?}", pr.schema()?);

    // let df_jan = pr.finish()?;
    // println!("{:?}", df_jan.schema());
    // println!("{}", df_jan.head(Some(5)));

    // basic::drill_1_print_schemea(&df_jan);
    // basic::drill_2_mean_trip_distance(&df_jan)?;
    // basic::drill_3_avg_tip_per_payment_type(df_jan)?;
    // basic::drill_4_avg_trip_distance_and_total_revenue_per_hour(df_jan)?;
    // basic::drill_5_trip_distance_greater_than(df_jan.lazy())?;
    // basic::drill_6_passenter_count_gt_3(df_jan.lazy())?;
    // basic::drill_7_trips_where_type_is_card_and_tip_is_gt_0(df_jan.lazy())?;
    // basic::drill_8_new_trip_percentage(df_jan.lazy())?;
    // basic::drill_9(df_jan.lazy())?;
    // basic::drill_10(df_jan.lazy())?;
    // basic::drill_11(df_jan.lazy())?;
    // basic::drill_12(df_jan.lazy())?;
    // basic::drill_13(df_jan.lazy())?;
    // basic::drill_14(df_jan.lazy())?;

    // extra::drill_1(df_jan.lazy())?;
    // extra::drill_2(df_jan.lazy())?;
    // extra::drill_3(df_jan.lazy())?;

    // let f = File::open("./data/yellow_tripdata_2024-02.parquet")?;
    // let pr = ParquetReader::new(f);
    // let df_feb = pr.finish()?;
    // extra::drill_4(df_jan.lazy(), df_feb.lazy())?;

    // LazyFrame::scan_parquet to avoid reading the whole file in memory
    let lf_jan = LazyFrame::scan_parquet(
        PlRefPath::new("./data/yellow_tripdata_2024-01.parquet"),
        Default::default(),
    )?;
    extra::drill_3(lf_jan)?;

    Ok(())
}
