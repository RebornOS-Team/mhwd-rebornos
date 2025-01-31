// Copyright (C) 2023 Vladislav Nepogodin
//
// This file is part of CachyOS chwd.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use std::path::Path;
use std::sync::Arc;

use crate::profile::Profile;
use crate::{consts, data};

#[derive(Debug, PartialEq)]
pub enum Transaction {
    Install,
    Remove,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Success,
    ErrorNotInstalled,
    ErrorAlreadyInstalled,
    ErrorNoMatchLocalConfig,
    ErrorScriptFailed,
    ErrorSetDatabase,
}

#[derive(Debug)]
pub enum Message {
    InstallStart,
    InstallEnd,
    RemoveStart,
    RemoveEnd,
}

#[inline]
pub fn get_current_cmdname(cmd_line: &str) -> &str {
    if let Some(trim_pos) = cmd_line.rfind('/') {
        return cmd_line.get((trim_pos + 1)..).unwrap();
    }
    cmd_line
}

pub fn find_profile(profile_name: &str, profiles: &[Profile]) -> Option<Arc<Profile>> {
    let found_profile = profiles.iter().find(|x| x.name == profile_name);
    if let Some(found_profile) = found_profile {
        return Some(Arc::new(found_profile.clone()));
    }
    None
}

pub fn check_nvidia_card() {
    if !Path::new("/var/lib/mhwd/ids/pci/nvidia.ids").exists() {
        println!("No nvidia ids found!");
        return;
    }

    let data = data::Data::new();
    for pci_device in data.pci_devices.iter() {
        if pci_device.available_profiles.is_empty() {
            continue;
        }

        if pci_device.vendor_id == "10de" {
            println!("NVIDIA card found!");
            return;
        }
    }
}

pub fn check_environment() -> Vec<String> {
    let mut missing_dirs = vec![];

    if !Path::new(consts::RHWD_PCI_CONFIG_DIR).exists() {
        missing_dirs.push(consts::RHWD_PCI_CONFIG_DIR.to_owned());
    }
    if !Path::new(consts::RHWD_PCI_DATABASE_DIR).exists() {
        missing_dirs.push(consts::RHWD_PCI_DATABASE_DIR.to_owned());
    }

    missing_dirs
}
