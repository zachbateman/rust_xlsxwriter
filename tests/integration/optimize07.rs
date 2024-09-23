// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_constant_memory();

    let strings = [
        "_",
        "_x",
        "_x0",
        "_x00",
        "_x000",
        "_x0000",
        "_x0000_",
        "_x005F_",
        "_x000G_",
        "_X0000_",
        "_x000a_",
        "_x000A_",
        "_x0000__x0000_",
        "__x0000__",
    ];

    worksheet.write_column(0, 0, strings)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize07() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize07")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
