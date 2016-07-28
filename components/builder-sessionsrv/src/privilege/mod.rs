// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use dbcache::IndexSet;
use hab_net::oauth::github::GitHubClient;
use protocol::sessionsrv as proto;

use data_store::DataStore;
use error::Result;

bitflags! {
    pub flags FeatureFlags: u32 {
        const ADMIN = 0b00000001,
        const BUILDER = 0b00000010,
    }
}

pub fn associate_team(ds: &DataStore, flag: u32, team: u64) -> Result<()> {
    try!(ds.features.github.write(&team, flag));
    Ok(())
}

pub fn list_grants(ds: &DataStore, flag: u32) -> Result<Vec<u64>> {
    // let teams = try!(ds.features.github.find(&flag));
    // Ok(teams)
    Ok(vec![])
}

pub fn set_features(ds: &DataStore, gh: &GitHubClient, session: &mut proto::Session) -> Result<()> {
    let flags = FeatureFlags::empty();
    let teams = try!(gh.teams(session.get_token()));
    for team in teams {
        if let Some(raw_flag) = ds.features.github.find(&team.id).ok() {
            let flag = FeatureFlags::from_bits(raw_flag).unwrap();
            debug!("Granting feature flag={:?} for team={:?}", flag, team.name);
            flags | flag;
        }
    }
    session.set_flags(flags.bits());
    Ok(())
}
