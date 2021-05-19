import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import {
  CategoryParams,
  CreateCategoriesFixture,
  DeleteThreadsFixture,
  MoveThreadParams,
  MoveThreadsFixture,
  SetStickyThreadsFixture,
  StickyThreadsParams,
  ThreadRemovalInput,
  ThreadTitleUpdate,
  UpdateThreadTitlesFixture,
} from '../../fixtures/forum'
import { CreateThreadsFixture, ThreadParams } from '../../fixtures/forum/CreateThreadsFixture'
import { BuyMembershipHappyCaseFixture } from '../../fixtures/membership'
import _ from 'lodash'
import { ThreadId } from '@joystream/types/common'

export default async function threads({ api, query }: FlowProps): Promise<void> {
  const debug = Debugger(`flow:threads`)
  debug('Started')
  api.enableDebugTxLogs()

  // Create test member(s)
  const accounts = (await api.createKeyPairs(5)).map((kp) => kp.address)
  const buyMembershipFixture = new BuyMembershipHappyCaseFixture(api, query, accounts)
  await new FixtureRunner(buyMembershipFixture).run()
  const memberIds = buyMembershipFixture.getCreatedMembers()

  // Create some test categories first
  const categories: CategoryParams[] = [
    { title: 'Test 1', description: 'Test category 1' },
    { title: 'Test 2', description: 'Test category 2' },
  ]
  const createCategoriesFixture = new CreateCategoriesFixture(api, query, categories)
  await new FixtureRunner(createCategoriesFixture).run()
  const categoryIds = createCategoriesFixture.getCreatedCategoriesIds()

  // Create threads
  const threads: ThreadParams[] = categoryIds.reduce(
    (threadsArray, categoryId) =>
      threadsArray.concat(
        memberIds.map((memberId) => ({
          categoryId,
          asMember: memberId,
          title: `Thread ${categoryId}/${memberId}`,
          text: `Example thread of member ${memberId.toString()} in category ${categoryId.toString()}`,
        }))
      ),
    [] as ThreadParams[]
  )

  const createThreadsFixture = new CreateThreadsFixture(api, query, threads)
  const createThreadsRunner = new FixtureRunner(createThreadsFixture)
  await createThreadsRunner.runWithQueryNodeChecks()
  const threadIds = createThreadsFixture.getCreatedThreadsIds()

  // Move threads
  const threadCategoryUpdates: MoveThreadParams[] = threadIds.map((threadId, i) => ({
    threadId,
    categoryId: threads[i].categoryId,
    newCategoryId: categoryIds[(categoryIds.indexOf(threads[i].categoryId) + 1) % categoryIds.length],
  }))

  const moveThreadsFixture = new MoveThreadsFixture(api, query, threadCategoryUpdates)
  const moveThreadsRunner = new FixtureRunner(moveThreadsFixture)
  await moveThreadsRunner.run()
  const threadCategories = threadCategoryUpdates.map((u) => u.newCategoryId)
  const threadIdsByCategoryId = threadIds.reduce((ids, id, i) => {
    const categoryId = threadCategories[i].toString()
    return { ...ids, [categoryId]: [...(ids[categoryId] || []), id] }
  }, {} as Record<string, ThreadId[]>)

  // Set threads as sticky (2 per category)
  const stickyThreadsParams: StickyThreadsParams[] = categoryIds.reduce((paramsArr, categoryId, i) => {
    const threadIds = threadIdsByCategoryId[categoryId.toString()]
    return paramsArr.concat([
      { categoryId, stickyTreads: [threadIds[0], threadIds[1]] },
      { categoryId, stickyTreads: [threadIds[1], threadIds[2]] },
    ])
  }, [] as StickyThreadsParams[])

  const setStickyThreadsFixture = new SetStickyThreadsFixture(api, query, stickyThreadsParams)
  const setStickyThreadsRunner = new FixtureRunner(setStickyThreadsFixture)
  await setStickyThreadsRunner.run()

  // Update titles
  const titleUpdates = threadIds.reduce(
    (updates, threadId, i) =>
      updates.concat([
        { threadId, categoryId: threadCategories[i], newTitle: '' },
        { threadId, categoryId: threadCategories[i], newTitle: `Test updated title ${i}` },
      ]),
    [] as ThreadTitleUpdate[]
  )

  const updateThreadTitlesFixture = new UpdateThreadTitlesFixture(api, query, titleUpdates)
  const updateThreadTitlesRunner = new FixtureRunner(updateThreadTitlesFixture)
  await updateThreadTitlesRunner.run()

  // Remove threads
  // TODO: Should removing / moving threads also "unstick" them?
  const threadRemovals: ThreadRemovalInput[] = threadIds.map((threadId, i) => ({
    threadId,
    categoryId: threadCategories[i],
    hide: i >= 1, // Test both cases
  }))
  const removeThreadsFixture = new DeleteThreadsFixture(api, query, threadRemovals)
  const removeThreadsRunner = new FixtureRunner(removeThreadsFixture)
  await removeThreadsRunner.run()

  // Run compound query node checks
  await Promise.all([
    moveThreadsRunner.runQueryNodeChecks(),
    updateThreadTitlesRunner.runQueryNodeChecks(),
    removeThreadsRunner.runQueryNodeChecks(),
  ])

  debug('Done')
}
