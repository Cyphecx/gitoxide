use std::{path::Path, fs::File, io::Read};

use gix_hash::ObjectId;
use gix_object::bstr::BString;

use crate::{FetchHead, FetchHeadEntry};

//TODO: should probably take in a repository object and find the FETCH_HEAD file from there.
pub fn parse(path: impl AsRef<Path>) -> Result<FetchHead, std::io::Error> {
    let mut source = match File::open(&path) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    let mut contents = String::new();
    if source.read_to_string(&mut contents).is_err() {
         panic!("couldn't read open file at path");
    }

    let out = FetchHead{
        entries: contents.lines().map(parse_entry).collect(),
    };

    Ok(out)
}

fn parse_entry(str: &str) -> FetchHeadEntry {
    let tokens: Vec<&str> = str.split('\t').collect();

    let head: ObjectId = ObjectId::from_hex(tokens[0].as_bytes()).unwrap();
    let merge_status: bool = tokens[1] != "not-for-merge";
    let sub_tokens: Vec<&str> = tokens[2].split(' ').collect();
    let branch: BString = sub_tokens[1].to_string().replace('\'', "").into();
    let remote: BString = sub_tokens[3].into();
    FetchHeadEntry{
        head,
        merge_status,
        branch,
        remote,
    }

}
