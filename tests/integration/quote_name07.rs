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

    let sheet_name = "Sheet'1";

    let worksheet = workbook.add_worksheet().set_name(sheet_name)?;
    worksheet.set_repeat_rows(0, 1)?;

    // Add some test data for the chart(s).
    let data = [[1, 2, 3], [2, 4, 6], [3, 6, 9], [4, 8, 12], [5, 10, 15]];
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, col_data) in row_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *col_data)?;
        }
    }

    let mut chart = Chart::new(ChartType::Column);
    chart.add_series().set_values((sheet_name, 0, 0, 4, 0));
    chart.add_series().set_values((sheet_name, 0, 1, 4, 1));
    chart.add_series().set_values((sheet_name, 0, 2, 4, 2));

    // Set the chart axis ids to match the random values in the Excel file.
    chart.set_axis_ids(48135552, 54701056);

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_quote_name07() {
    let test_runner = common::TestRunner::new()
        .set_name("quote_name07")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
