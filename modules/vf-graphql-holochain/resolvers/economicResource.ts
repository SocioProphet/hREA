/**
 * Resolvers for EconomicResource fields
 *
 * @package: HoloREA
 * @since:   2019-10-31
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule, ById, ReadParams, ResourceSpecificationAddress, ProcessSpecificationAddress, AddressableIdentifier } from '../types'
import { mapZomeFn } from '../connection'

import {
  EconomicResource,
  ResourceSpecification,
  Unit,
  ProcessSpecification,
  Action,
  Maybe,
  EconomicResourceConnection,
  UnitResponse,
  ProcessSpecificationResponse,
  ResourceSpecificationResponse,
} from '@valueflows/vf-graphql'
import { EconomicResourceSearchInput } from './zomeSearchInputTypes'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  const hasMeasurement = -1 !== enabledVFModules.indexOf(VfModule.Measurement)
  const hasKnowledge = -1 !== enabledVFModules.indexOf(VfModule.Knowledge)

  const readResources = mapZomeFn<EconomicResourceSearchInput, EconomicResourceConnection>(dnaConfig, conductorUri, 'observation', 'economic_resource_index', 'query_economic_resources')
  const readUnit = mapZomeFn<ById, UnitResponse>(dnaConfig, conductorUri, 'specification', 'unit', 'get_unit')
  const readProcessSpecification = mapZomeFn<ReadParams, ProcessSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'process_specification', 'get_process_specification')
  const readAction = mapZomeFn<ById, Action>(dnaConfig, conductorUri, 'specification', 'action', 'get_action')
  const readResourceSpecification = mapZomeFn<ReadParams, ResourceSpecificationResponse>(dnaConfig, conductorUri, 'specification', 'resource_specification', 'get_resource_specification')

  return Object.assign(
    {
      containedIn: async (record: EconomicResource): Promise<Maybe<EconomicResource>> => {
        const resources = await readResources({ params: { contains: record.id } })
        if (!resources.edges || !resources.edges.length) {
          return null
        }
        return resources.edges.pop()!['node']
      },

      contains: async (record: EconomicResource): Promise<EconomicResource[]> => {
        const resources = await readResources({ params: { containedIn: record.id } })
        if (!resources.edges || !resources.edges.length) {
          return []
        }
        return resources.edges.map(({ node }) => node)
      },
    },
    (hasKnowledge ? {
      conformsTo: async (record: { conformsTo: ResourceSpecificationAddress }): Promise<ResourceSpecification> => {
        return (await readResourceSpecification({ address: record.conformsTo })).resourceSpecification
      },

      stage: async (record: { stage: ProcessSpecificationAddress }): Promise<ProcessSpecification> => {
        return (await readProcessSpecification({ address: record.stage })).processSpecification
      },

      state: async (record: { state: AddressableIdentifier }): Promise<Action> => {
        return (await readAction({ id: record.state }))
      },
    } : {}),
    (hasMeasurement ? {
      unitOfEffort: async (record: { unitOfEffort: AddressableIdentifier }): Promise<Maybe<Unit>> => {
        if (!record.unitOfEffort) {
          return null
        }
        return (await readUnit({ id: record.unitOfEffort })).unit
      },
    } : {}),
  )
}
