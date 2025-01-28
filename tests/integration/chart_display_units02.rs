// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartAxisDisplayUnitType, ChartType, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [10000000, 20000000, 30000000, 20000000, 10000000];
    worksheet.write_column(0, 0, data)?;

    let mut chart = Chart::new(ChartType::Column);
    chart.set_axis_ids(56159232, 61364096);
    chart.add_series().set_values(("Sheet1", 0, 0, 4, 0));

    chart
        .y_axis()
        .set_display_unit_type(ChartAxisDisplayUnitType::Hundreds)
        .set_display_units_visible(false);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_display_units02() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_display_units02")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
