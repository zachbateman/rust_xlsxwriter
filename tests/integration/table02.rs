// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Table, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let table = Table::new();

    let worksheet1 = workbook.add_worksheet();

    worksheet1.set_column_width(1, 10.288)?;
    worksheet1.set_column_width(2, 10.288)?;
    worksheet1.set_column_width(3, 10.288)?;
    worksheet1.set_column_width(4, 10.288)?;
    worksheet1.set_column_width(5, 10.288)?;
    worksheet1.set_column_width(6, 10.288)?;
    worksheet1.set_column_width(7, 10.288)?;
    worksheet1.set_column_width(8, 10.288)?;
    worksheet1.set_column_width(9, 10.288)?;

    worksheet1.add_table(2, 1, 10, 4, &table)?;
    worksheet1.add_table(9, 6, 15, 9, &table)?;
    worksheet1.add_table(17, 2, 24, 5, &table)?;

    let worksheet2 = workbook.add_worksheet();

    worksheet2.set_column_width(2, 10.288)?;
    worksheet2.set_column_width(3, 10.288)?;
    worksheet2.set_column_width(4, 10.288)?;
    worksheet2.set_column_width(5, 10.288)?;
    worksheet2.set_column_width(6, 10.288)?;
    worksheet2.set_column_width(7, 10.288)?;
    worksheet2.set_column_width(8, 10.288)?;
    worksheet2.set_column_width(9, 10.288)?;
    worksheet2.set_column_width(10, 10.288)?;
    worksheet2.set_column_width(11, 10.288)?;

    worksheet2.add_table(3, 8, 10, 11, &table)?;
    worksheet2.add_table(15, 2, 22, 7, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table02() {
    let test_runner = common::TestRunner::new()
        .set_name("table02")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}