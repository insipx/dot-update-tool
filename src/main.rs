// Copyright 2017-2019 Parity Technologies (UK) Ltd.
// This file is part of substrate-archive.

// substrate-archive is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// substrate-archive is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with substrate-archive.  If not, see <http://www.gnu.org/licenses/>.

mod types;

use types::LatestRelease;
use flate2::read::GzDecoder;
use std::io::Read;
use tar::Archive;
use cargo_lock::Lockfile;
use std::str::FromStr;
use anyhow::Result;
use std::collections::HashSet;
use termimad::*;


fn main() -> anyhow::Result<()> {
    let release = get_latest_release()?;
    let lockfile = get_latest_lockfile(&release)?;
    let revisions: Vec<String> = get_all_substrate_deps(lockfile).into_iter().collect();
    let skin = MadSkin::default();

    if revisions.len() == 1 {
        println!(
        "
        ----------------- Polkadot Update Tool -----------------
        Latest Polkadot Version: {}    
        Latest version is pinned to substrate revision: {}
        \n\n
        {}
        ----------------- Polkadot Update Tool -----------------
        ", release.name, revisions[0], skin.term_text(&release.body));
    } else {
        panic!("Detected that latest polkadot is pinned to multiple revisions. This should not happen under normal circumstances");
    }
    Ok(()) 
}


fn get_latest_lockfile(release: &LatestRelease) -> Result<Lockfile> {
    let tarball_gz = ureq::get(release.tarball_url.as_str())
        .timeout_connect(10_000)
        .call()
        .into_reader();
    let tarball = GzDecoder::new(tarball_gz);
   
    let mut lockfile_body = String::new();
    
    Archive::new(tarball).entries()?.find(|e| {
        e.as_ref().unwrap().path().expect("Entry has no path").ends_with("Cargo.lock")
    }).expect("No Lockfile in Latest Release")?.read_to_string(&mut lockfile_body)?;
    
    let lockfile = Lockfile::from_str(lockfile_body.as_str())?;
    Ok(lockfile)
}

fn get_all_substrate_deps(lockfile: Lockfile) -> HashSet<String> {
    let mut versions = HashSet::new();

    for git in lockfile.packages.iter().filter_map(|p| { 
        p.source.as_ref().map(|s| {
            if s.is_git() && s.url().path().matches("paritytech/substrate").count() > 0 {
                s.precise()
            } else {
                None
            }
        })
    }).filter_map(|s| s) {
        versions.insert(git.into());
    }
    versions
}

fn get_latest_release() -> Result<LatestRelease> {
    ureq::get("https://api.github.com/repos/paritytech/polkadot/releases/latest")
        .timeout_connect(10_000)
        .call()
        .into_json_deserialize::<LatestRelease>().map_err(Into::into)
}
