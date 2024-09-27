// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Table, TableColumn, Workbook, XlsxError};

// Write a table with a user specified header. This also tests whitespace in headers.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_low_memory();

    worksheet.set_column_range_width(2, 5, 10.288)?;

    let columns = vec![
        TableColumn::default(),
        TableColumn::default(),
        TableColumn::default(),
        TableColumn::new().set_header(" Column4 "),
    ];

    let table = Table::new().set_columns(&columns);

    worksheet.add_table(2, 2, 12, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize_table19_1() {
    let test_runner = common::TestRunner::new()
        .set_name("table19")
        .set_function(create_new_xlsx_file)
        .unique("optimize")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
