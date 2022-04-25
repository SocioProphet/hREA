/**
 * Resolvers for agreement fields
 *
 * @package: Holo-REA
 * @since:   2020-06-19
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'
import { mapZomeFn } from '../connection'

import {
  Agreement,
  Commitment,
  EconomicEvent,
} from '@valueflows/vf-graphql'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasObservation = -1 !== enabledVFModules.indexOf(VfModule.Observation)
  const hasPlanning = -1 !== enabledVFModules.indexOf(VfModule.Planning)

  const queryCommitments = mapZomeFn(dnaConfig, conductorUri, 'planning', 'commitment_index', 'query_commitments')
  const queryEvents = mapZomeFn(dnaConfig, conductorUri, 'observation', 'economic_event_index', 'query_economic_events')

  return Object.assign(
    (hasPlanning ? {
      commitments: async (record: Agreement): Promise<Commitment[]> => {
        const commitments = await queryCommitments({ params: { clauseOf: record.id } })
        if (!commitments.edges || !commitments.edges.length) {
          return []
        }
        return commitments.edges.map(({ node }) => node)
      },
    } : {}),
    (hasObservation ? {
      economicEvents: async (record: Agreement): Promise<EconomicEvent[]> => {
        const economicEvents = await queryEvents({ params: { realizationOf: record.id } })
        if (!economicEvents.edges || !economicEvents.edges.length) {
          return []
        }
        return economicEvents.edges.map(({ node }) => node)
      },
    } : {}),
  )
}
