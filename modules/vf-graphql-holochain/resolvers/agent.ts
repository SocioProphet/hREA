/**
 * Resolvers for agent fields
 *
 * @package: Holo-REA
 * @since:   2020-05-28
 */

import { DNAIdMappings, DEFAULT_VF_MODULES, VfModule } from '../types'

export default (enabledVFModules: VfModule[] = DEFAULT_VF_MODULES, dnaConfig: DNAIdMappings, conductorUri: string) => {
  return {
    __resolveType: (obj, ctx, info) => obj.__typename
  }
}
