// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartAxisLabelPosition, ChartType, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    worksheet.write_column(0, 0, [1, 2, 3, 4, 5])?;
    worksheet.write_column(0, 1, [10, 40, 50, 20, 10])?;
    worksheet.write_column(0, 2, [1, 2, 3, 4, 5, 6, 7])?;
    worksheet.write_column(0, 3, [30, 10, 20, 40, 30, 10, 20])?;

    let mut chart = Chart::new(ChartType::Line);
    chart.set_axis_ids(77034624, 77036544);
    chart.set_axis2_ids(95388032, 103040896);

    chart
        .add_series()
        .set_categories(("Sheet1", 0, 0, 4, 0))
        .set_values(("Sheet1", 0, 1, 4, 1));

    chart
        .add_series()
        .set_categories(("Sheet1", 0, 2, 6, 2))
        .set_values(("Sheet1", 0, 3, 6, 3))
        .set_secondary_axis(true);

    chart
        .x2_axis()
        .set_label_position(ChartAxisLabelPosition::NextTo)
        .set_hidden(false);

    chart
        .y2_axis()
        .set_crossing(rust_xlsxwriter::ChartAxisCrossing::Max);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_line08() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_line08")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
