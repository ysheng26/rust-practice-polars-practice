use std::fs::File;

use polars::prelude::*;

#[allow(unused)]
fn drill_1_print_schemea(df: &DataFrame) {
    println!("{:?}", df.schema());

    // Schema:
    // name: VendorID, field: Int32
    // name: tpep_pickup_datetime, field: Datetime(Nanoseconds, None)
    // name: tpep_dropoff_datetime, field: Datetime(Nanoseconds, None)
    // name: passenger_count, field: Int64
    // name: trip_distance, field: Float64
    // name: RatecodeID, field: Int64
    // name: store_and_fwd_flag, field: String
    // name: PULocationID, field: Int32
    // name: DOLocationID, field: Int32
    // name: payment_type, field: Int64
    // name: fare_amount, field: Float64
    // name: extra, field: Float64
    // name: mta_tax, field: Float64
    // name: tip_amount, field: Float64
    // name: tolls_amount, field: Float64
    // name: improvement_surcharge, field: Float64
    // name: total_amount, field: Float64
    // name: congestion_surcharge, field: Float64
    // name: Airport_fee, field: Float64
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

#[allow(unused)]
fn drill_3_avg_tip_per_payment_type(df: DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    // payment_type — 1=credit card, 2=cash, 3=no charge, 4=dispute

    let x = df
        .lazy()
        .group_by(["payment_type"])
        .agg([col("tip_amount").mean()])
        .sort(
            ["payment_type"],
            SortMultipleOptions::new().with_order_descending(false),
        );
    println!("{}", x.collect()?);
    Ok(())
}

#[allow(unused)]
fn drill_4_avg_trip_distance_and_total_revenue_per_hour(
    df: DataFrame,
) -> Result<(), Box<dyn std::error::Error>> {
    /*
     * 2 — A harder query: what is the average trip distance and total revenue (total_amount) per hour of day?
     * You'll need to extract the hour from tpep_pickup_datetime and group by it.
     */

    let x = df
        .lazy()
        .group_by([col("tpep_pickup_datetime").dt().hour().alias("hour")])
        .agg([col("total_amount").mean(), col("trip_distance").mean()])
        .sort(
            ["hour"],
            SortMultipleOptions::new().with_order_descending(false),
        );
    println!("{}", x.collect()?);
    Ok(())
}

#[allow(unused)]
fn drill_5_trip_distance_greater_than(lf: LazyFrame) -> Result<(), Box<dyn std::error::Error>> {
    // 5. Return all trips where trip_distance is greater than 10 miles.

    let x = lf.filter(col("trip_distance").gt(10)).sort(
        ["total_amount"],
        SortMultipleOptions::new().with_order_descending(true),
    );
    println!("{}", x.collect()?);
    Ok(())
}

#[allow(unused)]
fn drill_6_passenter_count_gt_3(lf: LazyFrame) -> Result<(), Box<dyn std::error::Error>> {
    // 6. Return all trips where passenger_count is greater than 3.

    let x = lf.filter(col("passenger_count").gt(3)).sort(
        ["passenger_count"],
        SortMultipleOptions::new().with_order_descending(true),
    );

    println!("{}", x.collect()?);
    Ok(())
}

#[allow(unused)]
fn drill_7_trips_where_type_is_card_and_tip_is_gt_0(
    lf: LazyFrame,
) -> Result<(), Box<dyn std::error::Error>> {
    // 7. Return all trips where payment_type is credit card (1) AND tip_amount is greater than 0.

    let x = lf
        .filter(col("payment_type").eq(1))
        .filter(col("tip_amount").gt(0))
        .sort(
            ["tip_amount"],
            SortMultipleOptions::new().with_order_descending(true),
        );

    let x = x.select([col("tip_amount")]);
    println!("{}", x.collect()?);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the parquet file here
    // print the schema
    let f = File::open("./data/yellow_tripdata_2024-01.parquet")?;
    let pr = ParquetReader::new(f);
    // println!("{:?}", pr.schema());

    let df = pr.finish()?;
    // println!("{:?}", df.schema());
    // println!("{}", df.head(Some(5)));

    // let _ = drill_1_print_schemea(&df);
    // let _ = drill_2_mean_trip_distance(&df);
    // let _ = drill_3_avg_tip_per_payment_type(df);
    // let _ = drill_4_avg_trip_distance_and_total_revenue_per_hour(df);
    // let _ = drill_5_trip_distance_greater_than(df.lazy());
    // let _ = drill_6_passenter_count_gt_3(df.lazy());
    let _ = drill_7_trips_where_type_is_card_and_tip_is_gt_0(df.lazy());

    Ok(())
}
