/**
 * Resolvers for EconomicEvent fields
 *
 * @package: HoloREA
 * @since:   2019-08-27
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'
import { extractEdges, mapZomeFn } from '../connection'

import {
  Agent,
  EconomicEvent,
  EconomicResource,
  Fulfillment,
  Satisfaction,
  Process,
  ResourceSpecification,
  Action,
  Agreement,
} from '@valueflows/vf-graphql'

import agentQueries from '../queries/agent'
import agreementQueries from '../queries/agreement'
import resourceQueries from '../queries/economicResource'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasAgent = -1 !== enabledVFModules.indexOf(VfModule.Agent)
  const hasKnowledge = -1 !== enabledVFModules.indexOf(VfModule.Knowledge)
  const hasPlanning = -1 !== enabledVFModules.indexOf(VfModule.Planning)
  const hasAgreement = -1 !== enabledVFModules.indexOf(VfModule.Agreement)

  const readFulfillments = mapZomeFn(dnaConfig, conductorUri, 'observation', 'fulfillment_index', 'query_fulfillments')
  const readSatisfactions = mapZomeFn(dnaConfig, conductorUri, 'observation', 'satisfaction_index', 'query_satisfactions')
  const readProcesses = mapZomeFn(dnaConfig, conductorUri, 'observation', 'process_index', 'query_processes')
  const readAction = mapZomeFn(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readResourceSpecification = mapZomeFn(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')
  const readAgent = agentQueries(dnaConfig, conductorUri)['agent']
  const readAgreement = agreementQueries(dnaConfig, conductorUri)['agreement']
  const readResource = resourceQueries(dnaConfig, conductorUri)['economicResource']

  return Object.assign(
    {
      inputOf: async (record: EconomicEvent): Promise<Process[]> => {
        const results = await readProcesses({ params: { inputs: record.id } })
        return results.edges.pop()['node']
      },

      outputOf: async (record: EconomicEvent): Promise<Process[]> => {
        const results = await readProcesses({ params: { outputs: record.id } })
        return results.edges.pop()['node']
      },

      resourceInventoriedAs: async (record: EconomicEvent): Promise<EconomicResource | null> => {
        if (!record.resourceInventoriedAs) return null
        return await readResource(record, { id: record.resourceInventoriedAs })
      },
    },
    (hasAgent ? {
      provider: async (record: EconomicEvent): Promise<Agent> => {
        return readAgent(record, { id: record.provider })
      },

      receiver: async (record: EconomicEvent): Promise<Agent> => {
        return readAgent(record, { id: record.receiver })
      },
    } : {}),
    (hasPlanning ? {
      fulfills: async (record: EconomicEvent): Promise<Fulfillment[]> => {
        const results = await readFulfillments({ params: { fulfilledBy: record.id } })
        return extractEdges(results)
      },

      satisfies: async (record: EconomicEvent): Promise<Satisfaction[]> => {
        const results = await readSatisfactions({ params: { satisfiedBy: record.id } })
        return extractEdges(results)
      },
    } : {}),
    (hasKnowledge ? {
      resourceConformsTo: async (record: EconomicEvent): Promise<ResourceSpecification> => {
        return (await readResourceSpecification({ address: record.resourceConformsTo })).resourceSpecification
      },

      action: async (record: EconomicEvent): Promise<Action> => {
        return (await readAction({ id: record.action }))
      },
    } : {}),
    (hasAgreement ? {
      realizationOf: async (record: EconomicEvent): Promise<Agreement> => {
        return readAgreement(record, { id: record.realizationOf })
      },
    } : {}),
  )
}
