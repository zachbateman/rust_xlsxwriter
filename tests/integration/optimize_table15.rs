// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Table, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_low_memory();

    worksheet.set_column_range_width(2, 5, 10.288)?;

    let table = Table::new();
    worksheet.add_table(1, 2, 5, 5, &table)?;

    worksheet.write(2, 2, "Foo")?;
    worksheet.write_row(2, 3, [1234, 2000, 4321])?;

    worksheet.write(3, 2, "Bar")?;
    worksheet.write_row(3, 3, [1256, 0, 4320])?;

    worksheet.write(4, 2, "Baz")?;
    worksheet.write_row(4, 3, [2234, 3000, 4332])?;

    worksheet.write(5, 2, "Bop")?;
    worksheet.write_row(5, 3, [1324, 1000, 4333])?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize_table15() {
    let test_runner = common::TestRunner::new()
        .set_name("table15")
        .set_function(create_new_xlsx_file)
        .unique("optimize")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
