// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Chart, ChartFont, ChartType, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test to demonstrate charts.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 2, 3], [2, 4, 6], [3, 6, 9], [4, 8, 12], [5, 10, 15]];
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, col_data) in row_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *col_data)?;
        }
    }

    let mut chart = Chart::new(ChartType::Bar);
    chart.set_axis_ids(45704704, 45716224);
    chart.add_series().set_values(("Sheet1", 0, 0, 4, 0));
    chart.add_series().set_values(("Sheet1", 0, 1, 4, 1));
    chart.add_series().set_values(("Sheet1", 0, 2, 4, 2));

    chart
        .title()
        .set_name("Title")
        .set_font(ChartFont::new().set_italic().unset_bold());

    chart
        .x_axis()
        .set_name("XXX")
        .set_name_font(ChartFont::new().set_italic().unset_bold())
        .set_font(ChartFont::new().set_size(11).set_bold().set_italic());
    chart
        .y_axis()
        .set_name("YYY")
        .set_name_font(ChartFont::new().set_bold().set_italic())
        .set_font(ChartFont::new().set_size(9).set_italic());

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_font03() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_font03")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
