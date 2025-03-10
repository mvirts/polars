use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --8<-- [start:dataframe]
    use std::fs::File;

    use chrono::prelude::*;

    let mut df: DataFrame = df!(
        "integer" => &[1, 2, 3],
        "date" => &[
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        ],
        "float" => &[4.0, 5.0, 6.0]
    )
    .unwrap();
    println!("{}", df);
    // --8<-- [end:dataframe]

    // --8<-- [start:csv]
    let mut file = File::create("docs/data/output.csv").expect("could not create file");
    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df)?;
    let df_csv = CsvReader::from_path("docs/data/output.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;
    println!("{}", df_csv);
    // --8<-- [end:csv]

    // --8<-- [start:csv2]
    let mut file = File::create("docs/data/output.csv").expect("could not create file");
    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df)?;
    let df_csv = CsvReader::from_path("docs/data/output.csv")?
        .infer_schema(None)
        .has_header(true)
        .with_try_parse_dates(true)
        .finish()?;
    println!("{}", df_csv);
    // --8<-- [end:csv2]

    // --8<-- [start:json]
    let mut file = File::create("docs/data/output.json").expect("could not create file");
    JsonWriter::new(&mut file).finish(&mut df)?;
    let f = File::open("docs/data/output.json")?;
    let df_json = JsonReader::new(f)
        .with_json_format(JsonFormat::JsonLines)
        .finish()?;
    println!("{}", df_json);
    // --8<-- [end:json]

    // --8<-- [start:parquet]
    let mut file = File::create("docs/data/output.parquet").expect("could not create file");
    ParquetWriter::new(&mut file).finish(&mut df)?;
    let f = File::open("docs/data/output.parquet")?;
    let df_parquet = ParquetReader::new(f).finish()?;
    println!("{}", df_parquet);
    // --8<-- [end:parquet]

    Ok(())
}
