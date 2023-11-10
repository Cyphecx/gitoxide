#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

use gix_hash::ObjectId;
use gix_object::bstr::BString;

/// Represents parsed data from the FETCH_HEAD file.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct FetchHeadEntry {
    //Head of fetched repository
    pub head: gix_hash::ObjectId,

    //Merge status
    pub merge_status: bool,

    //Branch name
    pub branch: BString,

    //Remote url
    pub remote: BString,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct FetchHead {
    //List of fetch entries contained in the FETCH_HEAD FILE
    pub entries: Vec<FetchHeadEntry>,
}

impl FetchHead {

    pub fn add_entry(&mut self, head: ObjectId, merge_status: bool, branch: BString, remote: BString) {
        let new_entry = FetchHeadEntry {
            head,
            merge_status,
            branch,
            remote,
        };
        self.entries.push(new_entry);
    }

}
pub mod parse;
pub mod write;
