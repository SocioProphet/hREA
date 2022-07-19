import test from 'tape'
import { pause } from '@holochain/tryorama'
import {
  mockAddress, mockIdentifier,
  buildPlayer,
} from '../init.js'

const testEventProps = {
  action: 'raise',
  provider: mockAddress(),
  receiver: mockAddress(),
  resourceQuantity: { hasNumericalValue: 1, hasUnit: mockIdentifier() },
}

test('can locate EconomicResources conforming to a ResourceSpecification', async (t) => {
  const alice = await buildPlayer(['observation', 'specification'])
  try {
    const { graphQL } = alice

    let resp = await graphQL(`
      mutation(
        $rs: ResourceSpecificationCreateParams!,
      ) {
        rs: createResourceSpecification(resourceSpecification: $rs) {
          resourceSpecification {
            id
          }
        }
      }
    `, {
      rs: {
        name: 'test resource spec',
      },
    })
    await pause(100)

    t.ok(resp.data.rs.resourceSpecification.id, 'ResourceSpecification created')
    const rsId = resp.data.rs.resourceSpecification.id

    resp = await graphQL(`
      mutation(
        $e1: EconomicEventCreateParams!,
        $r1: EconomicResourceCreateParams!,
        $e2: EconomicEventCreateParams!,
        $r2: EconomicResourceCreateParams!,
      ) {
        r1: createEconomicEvent(event: $e1, newInventoriedResource: $r1) {
          economicResource {
            id
          }
        }
        r2: createEconomicEvent(event: $e2, newInventoriedResource: $r2) {
          economicResource {
            id
          }
        }
      }
    `, {
      e1: {
        resourceConformsTo: rsId,
        hasPointInTime: '2019-11-19T04:29:55.000Z',
        ...testEventProps,
      },
      r1: { note: 'resource A' },
      e2: {
        resourceConformsTo: rsId,
        hasPointInTime: '2019-11-19T04:29:56.000Z',
        ...testEventProps,
      },
      r2: { note: 'resource B' },
    })
    await pause(100)

    t.ok(resp.data.r1.economicResource.id, 'first resource created')
    t.ok(resp.data.r2.economicResource.id, 'second resource created')
    const resource1Id = resp.data.r1.economicResource.id
    const resource2Id = resp.data.r2.economicResource.id

    resp = await graphQL(`{
      rs: resourceSpecification(id: "${rsId}") {
        conformingResources {
          edges {
            node {
              id
            }
          }
        }
      }
    }`)

    t.equal(resp.data.rs.conformingResources.edges.length, 2, 'all resources indexed via ResourceSpecification link')
    t.equal(resp.data.rs.conformingResources.edges[0].node.id, resource2Id, 'resource 2 ref OK')
    t.equal(resp.data.rs.conformingResources.edges[1].node.id, resource1Id, 'resource 1 ref OK')
  } catch (e) {
    await alice.scenario.cleanUp()
    throw e
  }
  await alice.scenario.cleanUp()
})
