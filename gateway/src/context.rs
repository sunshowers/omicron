// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::error::StartupError;
use crate::management_switch::ManagementSwitch;
use crate::management_switch::SwitchConfig;
use gateway_sp_comms::InMemoryHostPhase2Provider;
use slog::Logger;
use std::sync::Arc;

/// Shared state used by API request handlers
pub struct ServerContext {
    pub mgmt_switch: ManagementSwitch,
    pub host_phase2_provider: Arc<InMemoryHostPhase2Provider>,
    pub log: Logger,
}

impl ServerContext {
    pub async fn new(
        host_phase2_provider: Arc<InMemoryHostPhase2Provider>,
        switch_config: SwitchConfig,
        log: &Logger,
    ) -> Result<Arc<Self>, StartupError> {
        let mgmt_switch =
            ManagementSwitch::new(switch_config, &host_phase2_provider, log)
                .await?;

        Ok(Arc::new(ServerContext {
            mgmt_switch,
            host_phase2_provider,
            log: log.clone(),
        }))
    }
}
