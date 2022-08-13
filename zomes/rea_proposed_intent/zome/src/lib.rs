/**
* Holo-REA proposed intents zome API definition
*
* Defines the top-level zome configuration needed by Holochain's build system
* to bundle the app. This basically involves wiring up the helper methods from the
* related `_lib` module into a packaged zome WASM binary.
*
* @package Holo-REA
*/
use hdk::prelude::*;

use hc_zome_rea_proposed_intent_lib::*;
use hc_zome_rea_proposed_intent_rpc::*;
use hc_zome_rea_proposed_intent_storage_consts::*;

#[hdk_extern]
fn create_proposed_intent(CreateParams { proposed_intent }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_proposed_intent(PROPOSED_INTENT_ENTRY_TYPE, proposed_intent)?)
}

#[hdk_extern]
fn get_proposed_intent(ByAddress { address }: ByAddress<ProposedIntentAddress>) -> ExternResult<ResponseData> {
    Ok(handle_get_proposed_intent(address)?)
}

#[hdk_extern]
fn delete_proposed_intent(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_proposed_intent(&revision_id)?)
}
