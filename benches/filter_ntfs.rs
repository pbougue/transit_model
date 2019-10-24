// Copyright (C) 2017 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU Affero General Public License as published by the
// Free Software Foundation, version 3.

// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>

#![feature(test)]

extern crate test;

use test::Bencher;
use transit_model::{
    ntfs,
    ntfs::{filter, filter::Action::*},
};

#[bench]
fn filter_ntfs_extract(bencher: &mut Bencher) {
    bencher.iter(|| {
        filter::filter(
            ntfs::read("./tests/fixtures/filter_ntfs/input").unwrap(),
            Extract,
            vec![String::from("network1")],
        )
    });
}

#[bench]
fn filter_ntfs_remove(bencher: &mut Bencher) {
    bencher.iter(|| {
        filter::filter(
            ntfs::read("./tests/fixtures/filter_ntfs/input").unwrap(),
            Remove,
            vec![String::from("network1")],
        )
    });
}
