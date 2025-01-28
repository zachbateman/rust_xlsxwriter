// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartFormat, ChartSolidFill, ChartType, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[2, 4, 6], [60, 30, 10]];
    for (col_num, col_data) in data.iter().enumerate() {
        for (row_num, row_data) in col_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *row_data)?;
        }
    }

    let mut chart = Chart::new(ChartType::Pie);
    chart
        .add_series()
        .set_categories(("Sheet1", 0, 0, 2, 0))
        .set_values(("Sheet1", 0, 1, 2, 1));

    chart
        .legend()
        .set_format(ChartFormat::new().set_solid_fill(ChartSolidFill::new().set_color("#FFFF00")));

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_legend07() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_legend07")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
