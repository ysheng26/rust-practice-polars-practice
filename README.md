# Rust Polars Practice Drills

This repository contains a series of drills designed to practice data manipulation using the Rust Polars library. The drills use the NYC Yellow Taxi Parquet dataset.

## Basic Drills (`src/basic.rs`)

1. **Print Schema:** Print the schema of the DataFrame.
2. **Mean Trip Distance:** Find the mean `trip_distance`.
3. **Average Tip per Payment Type:** What is the average `tip_amount` per `payment_type`?
4. **Hourly Averages:** What is the average `trip_distance` and total revenue (`total_amount`) per hour of day?
5. **Long Trips:** Return all trips where `trip_distance` is greater than 10 miles.
6. **Large Groups:** Return all trips where `passenger_count` is greater than 3.
7. **Credit Card Tips:** Return all trips where `payment_type` is credit card (1) AND `tip_amount` is greater than 0.
8. **Tip Percentage:** Add a new column `tip_percentage` calculated as `tip_amount / fare_amount * 100`.
9. **Trip Duration:** Add a new column `trip_duration_minutes` calculated from the difference between dropoff and pickup datetimes.
10. **Trips per Hour:** Count the number of trips per hour of day, sorted by hour ascending.
11. **Busiest Zones:** Find the top 10 busiest pickup zones (`PULocationID`) by trip count.
12. **Tip % by Payment Type:** Find the average `tip_percentage` per payment type.
13. **Hourly Tip % (Credit Only):** For credit card trips only, what is the average tip percentage per hour of day?
14. **Rush Hour Revenue:** What are the top 5 pickup zones by total revenue during rush hour (7am-10am and 5pm-8pm)?

---

## Extra Drills (`src/extra.rs`)

**E1. Rolling Averages:**
What is the 7-day rolling average of daily trip count for January 2024?

**E2. Conditional Aggregation:**
For each pickup zone, what percentage of trips are cash vs credit card? The result should have one row per zone with columns `PULocationID`, `cash_pct`, `credit_pct`.

**E3. Window Functions:**
Find all trips where the `tip_amount` is higher than the average tip for that hour of day. This requires computing a per-group statistic without collapsing the rows like `group_by` does.

**E4. Concatenation & Multi-file Analysis:**
Load January and February 2024 data, concatenate them into one DataFrame, and find the top 10 busiest pickup zones across both months combined.
