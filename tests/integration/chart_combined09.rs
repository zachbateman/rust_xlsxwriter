// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{
    Chart, ChartAxisLabelPosition, ChartEmptyCells, ChartFont, ChartType, Workbook, XlsxError,
};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    worksheet.write_column(0, 0, [2, 7, 3, 6, 2])?;
    worksheet.write_column(0, 1, [20, 25, 10, 10, 20])?;

    let mut chart1 = Chart::new(ChartType::Column);
    chart1.set_axis_ids(114984064, 114985600);
    chart1.set_axis2_ids(114988928, 114987392);

    chart1.add_series().set_values(("Sheet1", 0, 0, 4, 0));

    chart1
        .y_axis()
        .set_font(ChartFont::new().set_bold().set_default_bold(true));
    chart1
        .y2_axis()
        .set_font(ChartFont::new().set_bold().set_default_bold(true));

    chart1.show_empty_cells_as(ChartEmptyCells::Gaps);

    let mut chart2 = Chart::new(ChartType::Line);
    chart2
        .add_series()
        .set_values(("Sheet1", 0, 1, 4, 1))
        .set_secondary_axis(true);

    chart1.combine(&chart2);

    chart1
        .x2_axis()
        .set_label_position(ChartAxisLabelPosition::NextTo);

    worksheet.insert_chart(8, 4, &chart1)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_combined09() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_combined09")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
