// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartType, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    worksheet.write_column(0, 0, [1, 2, 3, 4, 5])?;
    worksheet.write_column(0, 1, [6, 8, 6, 4, 2])?;

    let mut chart = Chart::new(ChartType::Column);
    chart.set_axis_ids(45938176, 59715584);
    chart.set_axis2_ids(62526208, 59718272);

    chart
        .add_series()
        .set_values(("Sheet1", 0, 0, 4, 0))
        .set_overlap(12)
        .set_gap(51);

    chart
        .add_series()
        .set_values(("Sheet1", 0, 1, 4, 1))
        .set_overlap(-27)
        .set_gap(251)
        .set_secondary_axis(true);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_gap04() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_gap04")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
