// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test to demonstrate escaping control characters in strings.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    for i in 0u8..127 {
        worksheet.write_string(i as u32, 0, (i as char).to_string())?;
    }

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_shared_strings01() {
    let test_runner = common::TestRunner::new()
        .set_name("shared_strings01")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
