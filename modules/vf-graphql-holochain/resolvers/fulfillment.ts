/**
 * Resolvers for Fulfillment fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, injectTypename, DEFAULT_VF_MODULES, VfModule } from '../types'
import { mapZomeFn } from '../connection'

import {
  Fulfillment,
  EconomicEvent,
  Commitment,
} from '@valueflows/vf-graphql'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)

  const readEvents = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event_index', 'query_economic_events')
  const readCommitments = mapZomeFn(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')

  return Object.assign(
    {
      fulfills: injectTypename('Commitment', async (record: Fulfillment): Promise<Commitment> => {
        return (await readCommitments({ params: { fulfilledBy: record.id } })).pop()['commitment']
      }),
    },
    (hasObservation ? {
      fulfilledBy: injectTypename('EconomicEvent', async (record: Fulfillment): Promise<EconomicEvent> => {
        return (await readEvents({ params: { fulfills: record.id } })).pop()['economicEvent']
      }),
    } : {}),
  )
}
