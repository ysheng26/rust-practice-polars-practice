use polars::prelude::*;
/*
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

    // chrome hijacking ctrl-g even when not being focused

    println!("{}", x.collect()?);
    Ok(())
}

#[allow(unused)]
pub fn drill_2(lf: LazyFrame) -> Result<(), Box<dyn std::error::Error>> {
    // E2: For each pickup zone, what percentage of trips are cash vs credit card?
    // The result should have one row per zone
    // with columns PULocationID, cash_pct, credit_pct.

    // it transforms the rows, does not filter them
    // [1, 2, 1, 1, 2].eq(1) -> [true, false, true, true, false]
    // cast(DataType::Float64)
    // [true, false, true, true, false] -> [1.0, 0,0, 1,0, 1.0, 0.0]
    //
    // But if dataframes only transform, doesn't filter remove rows?
    // yes it does, but it filters on the [true, false, ture]
    //
    // replay mental model

    let x = lf
        .filter(
            col("payment_type")
                .eq(lit(1))
                .or(col("payment_type").eq(lit(2))),
        )
        .group_by([col("PULocationID")])
        .agg([
            col("payment_type")
                .eq(lit(1))
                .cast(DataType::Float64)
                .mean()
                .alias("credit_pct"),
            col("payment_type")
                .eq(lit(2))
                .cast(DataType::Float64)
                .mean()
                .alias("cash_pct"),
        ]);

    // after group_by, do I have access over the previous shape?
    // no
    // group_by would collapse the rows
    //
    // over() would keep the previous shape
    // group_by column and aggreegate
    // do this over

    println!("{}", x.collect()?);
    Ok(())
}
