// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{cell_autofit_width, Workbook, XlsxError};

// Test to demonstrate autofit.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_string(0, 0, "A")?;
    worksheet.write_string(0, 1, "A")?;

    worksheet.autofit();

    worksheet.set_column_width(1, 1.57)?;

    workbook.save(filename)?;

    Ok(())
}

// Test implicit version autofit width.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_string(0, 0, "A")?;
    worksheet.write_string(0, 1, "A")?;

    worksheet.set_column_width(1, 1.57143)?;

    worksheet.autofit();
    worksheet.autofit();

    workbook.save(filename)?;

    Ok(())
}

// Test with the Utility::autofix_cell_width() function.
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_string(0, 0, "A")?;
    worksheet.write_string(0, 1, "A")?;

    let max_col_width = cell_autofit_width("A");
    worksheet.set_column_autofit_width(0, max_col_width)?;

    worksheet.set_column_width(1, 1.57143)?;

    workbook.save(filename)?;

    Ok(())
}

// Test with the autofit_to_max_width() above Excel max.
fn create_new_xlsx_file_4(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_string(0, 0, "A")?;
    worksheet.write_string(0, 1, "A")?;

    worksheet.set_column_width(1, 1.57143)?;

    worksheet.autofit_to_max_width(2000);

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_autofit03_1() {
    let test_runner = common::TestRunner::new()
        .set_name("autofit03")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_autofit03_2() {
    let test_runner = common::TestRunner::new()
        .set_name("autofit03")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_autofit03_3() {
    let test_runner = common::TestRunner::new()
        .set_name("autofit03")
        .set_function(create_new_xlsx_file_3)
        .unique("3")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_autofit03_4() {
    let test_runner = common::TestRunner::new()
        .set_name("autofit03")
        .set_function(create_new_xlsx_file_4)
        .unique("4")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
