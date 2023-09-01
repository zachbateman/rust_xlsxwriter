// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test case to demonstrate creating a basic file with user defined column.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.set_column_width(2, 6.86)?;
    worksheet.set_column_width(1, 6.86)?;
    worksheet.set_column_width(0, 14)?; // Unsorted order.
    worksheet.set_column_width(0, 14)?; // Overwrite existing value.
    worksheet.set_column_width(3, 0.92)?;
    worksheet.set_column_width(4, 0.92)?;
    worksheet.set_column_width(5, 0.92)?;

    worksheet.write_number(8, 0, 123)?; // A cell to test dimensions.

    workbook.save(filename)?;

    Ok(())
}

// Test case to demonstrate creating a basic file with user defined column.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.set_column_width_pixels(2, 53)?;
    worksheet.set_column_width_pixels(1, 53)?;
    worksheet.set_column_width_pixels(0, 103)?; // Unsorted order.
    worksheet.set_column_width_pixels(3, 11)?;
    worksheet.set_column_width_pixels(4, 11)?;
    worksheet.set_column_width_pixels(5, 11)?;

    worksheet.write_number(8, 0, 123)?; // A cell to test dimensions.

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap25_set_column() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap25")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap25_set_column_pixels() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap25")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}