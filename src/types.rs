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
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LatestRelease {
    pub url: String,
    pub id: usize,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub assets: Vec<ReleaseAsset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReleaseAsset {
    pub url: String,
    pub id: usize,
    pub node_id: String,
    pub name: String,
    pub download_count: usize,
    pub content_type: String,
    pub size: usize,
    pub browser_download_url: String,
}
