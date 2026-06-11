use polars::prelude::*;
/*
E2
For each pickup zone, what percentage of trips are cash vs credit card? The result should have one row per zone with columns PULocationID, cash_pct, credit_pct.

E3
Find all trips where the tip_amount is higher than the average tip for that hour of day. This requires a window function — computing a per-group statistic without collapsing the rows like group_by does.

E4
Load January and February 2024 data, concatenate them into one DataFrame,
and find the top 10 busiest pickup zones across both months combined. February file under data/
*/

#[allow(unused)]
pub fn drill_1(lf: LazyFrame) -> Result<(), Box<dyn std::error::Error>> {
    // E1: What is the 7-day rolling average of daily trip count for January 2024?
    // You'll need to group by date first, then apply a rolling window.
    let x = lf
        .filter(col("tpep_pickup_datetime").dt().month().eq(lit(1)))
        .filter(col("tpep_pickup_datetime").dt().year().eq(lit(2024)))
        .with_column(col("tpep_pickup_datetime").dt().date().alias("date"))
        .group_by([col("date")])
        .agg([len().alias("trip_count")])
        .sort(
            ["date"],
            SortMultipleOptions::new().with_order_descending(false),
        )
        .collect()?;

    // rolling_mean is in crate feature rolling_window
    // and this requires "strings" (agy found it out for me)
    // maybe I should have used cargo to add the feature instead of hand changing Cargo.toml
    let x = x.lazy().with_column(
        col("trip_count")
            .rolling_mean(RollingOptionsFixedWindow {
                // rolling_mean takes self, does that mean it consumes the Expr?
                // does that mean Expr is Copy or Clone?
                // Expr is Clone not Copy
                // it doesn't matter here because Expr is being consumed
                window_size: 7,
                min_periods: 7,
                ..Default::default() // how does this actually work? `impl Default for RollingOptionsFixedWindow`
            })
            .alias("trip_count_rolling_7d"),
    );

    // how does Expr work?

    println!("{}", x.collect()?);
    Ok(())
}
