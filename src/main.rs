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

#[allow(unused)]
fn drill_8_new_trip_percentage(lf: LazyFrame) -> Result<(), Box<dyn std::error::Error>> {
    // 8. Add a new column tip_percentage calculated as tip_amount / fare_amount * 100. Print the first 5 rows showing only fare_amount, tip_amount, and tip_percentage.

    let x = lf
        .filter(col("fare_amount").gt(2.5))
        .with_column((col("tip_amount") / col("fare_amount") * lit(100)).alias("tip_percentage"))
        .select([col("fare_amount"), col("tip_amount"), col("tip_percentage")])
        .sort(
            ["tip_percentage"],
            SortMultipleOptions::new().with_order_descending(true),
        )
        .limit(20);
    println!("{}", x.collect()?);
    Ok(())
}

#[allow(unused)]
fn drill_9(lf: LazyFrame) -> Result<(), Box<dyn std::error::Error>> {
    /*
     * Drill 9 — Add a new column trip_duration_minutes calculated from the difference between tpep_dropoff_datetime and tpep_pickup_datetime. Print the first 5 rows showing only tpep_pickup_datetime, tpep_dropoff_datetime, and trip_duration_minutes.
     */
    let x = lf
        .with_column(
            ((col("tpep_dropoff_datetime") - col("tpep_pickup_datetime"))
                .dt()
                .total_minutes())
            .alias("trip_duration_minutes"),
        )
        .select([
            col("tpep_pickup_datetime"),
            col("tpep_dropoff_datetime"),
            col("trip_duration_minutes"),
        ])
        .limit(5);
    println!("{}", x.collect()?);
    // println!("{:?}", x.collect()?.schema());

    Ok(())
}

#[allow(unused)]
fn drill_10(lf: LazyFrame) -> Result<(), Box<dyn std::error::Error>> {
    /*
    * Drill 10 — Count the number of trips per hour of day, sorted by hour ascending.
    * You've done this pattern before in drill 4 — extract the hour from tpep_pickup_datetime, group by it, but this time aggregate with a count instead of mean/sum.
    Look for .count() in the Polars expression API.
    */

    let x = lf
        .group_by([col("tpep_pickup_datetime").dt().hour().alias("hour")])
        // .agg([col("tpep_pickup_datetime").count()])
        .agg([len()])
        .sort(
            ["hour"],
            SortMultipleOptions::new().with_order_descending(false),
        );

    println!("{}", x.collect()?);

    Ok(())
}

#[allow(unused)]
fn drill_11(lf: LazyFrame) -> Result<(), Box<dyn std::error::Error>> {
    // Drill 11 — Find the top 10 busiest pickup zones (PULocationID) by trip count.
    // You have everything you need from drill 10 — same pattern, different column. Group by PULocationID, count rows per group, sort descending, take top 10.

    let x = lf
        .group_by([col("PULocationID")])
        .agg([len().alias("count")])
        .sort(
            ["count"],
            SortMultipleOptions::new().with_order_descending(true),
        )
        .limit(10);
    println!("{}", x.collect()?);
    Ok(())
}

#[allow(unused)]
fn drill_12(lf: LazyFrame) -> Result<(), Box<dyn std::error::Error>> {
    // Drill 12 — Find the average tip_percentage per payment type.
    // You'll need to combine drill 8 (creating the tip_percentage column with with_column) and drill 3 (grouping by payment_type and aggregating). Chain them together in one lazy query.

    let x = lf
        .filter(col("fare_amount").gt(2.5))
        .with_column((col("tip_amount") / col("fare_amount") * lit(100)).alias("tip_percentage"))
        .group_by([col("payment_type")])
        .agg([col("tip_percentage").mean()])
        .sort(
            ["tip_percentage"],
            SortMultipleOptions::new().with_order_descending(true),
        );
    println!("{}", x.collect()?);

    Ok(())
}

#[allow(unused)]
fn drill_13(lf: LazyFrame) -> Result<(), Box<dyn std::error::Error>> {
    // 13. For credit card (payment type 1) trips only, what is the average tip percentage per hour of day? Sort by hour.

    let x = lf
        .filter(col("payment_type").eq(lit(1)))
        .filter(col("fare_amount").gt(lit(2.5)))
        .with_column((col("tip_amount") / col("fare_amount") * lit(100)).alias("tip_percentage"))
        .group_by([col("tpep_pickup_datetime").dt().hour().alias("hour")])
        .agg([col("tip_percentage").mean()])
        .sort(
            ["hour"],
            SortMultipleOptions::new().with_order_descending(false),
        );

    println!("{}", x.collect()?);
    Ok(())
}

#[allow(unused)]
fn drill_14(lf: LazyFrame) -> Result<(), Box<dyn std::error::Error>> {
    // 14. What are the top 5 pickup zones by total revenue (total_amount) during rush hour (7am-10am and 5pm-8pm)?
    let x = lf
        .with_columns([col("tpep_pickup_datetime").dt().hour().alias("hour")])
        .filter(
            (col("hour").gt_eq(lit(7)).and(col("hour").lt_eq(lit(10))))
                .or(col("hour").gt_eq(lit(17)).and(col("hour").lt_eq(lit(20)))),
        )
        .group_by(["PULocationID"])
        .agg([col("total_amount").sum()])
        .sort(
            ["total_amount"],
            SortMultipleOptions::new().with_order_descending(true),
        )
        .limit(5);

    println!("{}", x.collect()?);
    Ok(())
}

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

    // drill_1_print_schemea(&df);
    // drill_2_mean_trip_distance(&df)?;
    // drill_3_avg_tip_per_payment_type(df)?;
    // drill_4_avg_trip_distance_and_total_revenue_per_hour(df)?;
    // drill_5_trip_distance_greater_than(df.lazy())?;
    // drill_6_passenter_count_gt_3(df.lazy())?;
    // drill_7_trips_where_type_is_card_and_tip_is_gt_0(df.lazy())?;
    // drill_8_new_trip_percentage(df.lazy())?;
    // drill_9(df.lazy())?;
    // drill_10(df.lazy())?;
    // drill_11(df.lazy())?;
    // drill_12(df.lazy())?;
    // drill_13(df.lazy())?;
    drill_14(df.lazy())?;
    Ok(())
}
