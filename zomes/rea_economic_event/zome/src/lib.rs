/**
 * REA `EconomicEvent` zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_economic_event_zome_api::*;
use hc_zome_rea_economic_event_lib::*;
use hc_zome_rea_economic_event_storage::*;
use hc_zome_rea_process_storage_consts::PROCESS_ENTRY_TYPE;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        EconomicEventAddress::entry_def(),
        EntryDef {
            id: EVENT_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            required_validations: 2.into(),
        },
        EntryDef {
          id: CAP_STORAGE_ENTRY_DEF_ID.into(),
          visibility: EntryVisibility::Private,
          required_validations: 1.into(),
        }
    ]))
}

declare_economic_event_zome_validation_defaults!();
declare_economic_event_zome_api!(EconomicEventZomePermissableDefault);
