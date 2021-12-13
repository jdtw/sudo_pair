// Copyright 2018 Square Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied. See the License for the specific language governing
// permissions and limitations under the License.

use std::result::Result as StdResult;
use thiserror::Error;

use sudo_plugin::prelude::{Error as PluginError, LogStatus, OpenStatus};

pub(crate) type Result<T> = StdResult<T, Error>;

// PluginError is larger than io::Error, but this shouldn't be a
// problem for this error enum.
#[allow(variant_size_differences)]
#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("couldn't establish communications with the pair")]
    Communication(#[from] std::io::Error),

    #[error("pair declined the session")]
    SessionDeclined,

    #[error("pair ended the session")]
    SessionTerminated,

    #[error("redirection of stdin to paired sessions is prohibited")]
    StdinRedirected,

    #[error("the -u and -g options may not both be specified")]
    SudoToUserAndGroup,

    #[error("the plugin failed to initialize")]
    Plugin(#[from] PluginError),
}

impl From<Error> for OpenStatus {
    fn from(_: Error) -> Self {
        OpenStatus::Deny
    }
}

impl From<Error> for LogStatus {
    fn from(_: Error) -> Self {
        LogStatus::Deny
    }
}
