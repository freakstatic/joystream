import { Api } from '../../Api'
import { FlowProps } from '../../Flow'
import { CreateVideoFixture } from '../../fixtures/contentDirectoryModule'
import { VideoEntity } from '@joystream/cd-schemas/types/entities/VideoEntity'
import { assert } from 'chai'
import { Utils } from '../../utils'
import { FixtureRunner } from '../../Fixture'
import { extendDebug } from '../../Debugger'

export function createVideoReferencingChannelFixture(api: Api, handle: string): CreateVideoFixture {
  const videoEntity: VideoEntity = {
    title: 'Example joyjoy',
    description: 'This is an example joyjoy',
    // We reference existing language and category by their unique properties with "existing" syntax
    // (those referenced here are part of inputs/entityBatches)
    language: { existing: { code: 'EN' } },
    category: { existing: { name: 'Education' } },
    // We use the same "existing" syntax to reference a channel by unique property (handle)
    // In this case it's a channel that we created in createChannel example
    channel: { existing: { handle } },
    media: {
      // We use "new" syntax to sygnalize we want to create a new VideoMedia entity that will be related to this Video entity
      new: {
        // We use "exisiting" enconding from inputs/entityBatches/VideoMediaEncodingBatch.json
        encoding: { existing: { name: 'H.263_MP4' } },
        pixelHeight: 600,
        pixelWidth: 800,
        // We create nested VideoMedia->MediaLocation->HttpMediaLocation relations using the "new" syntax
        location: { new: { httpMediaLocation: { new: { url: 'https://testnet.joystream.org/' } } } },
      },
    },
    // Here we use combined "new" and "existing" syntaxes to create Video->License->KnownLicense relations
    license: {
      new: {
        knownLicense: {
          // This license can be found in inputs/entityBatches/KnownLicenseBatch.json
          existing: { code: 'CC_BY' },
        },
      },
    },
    duration: 3600,
    thumbnailUrl: 'url3',
    isExplicit: false,
    isPublic: false,
  }
  return new CreateVideoFixture(api, videoEntity)
}

function assertVideoMatchQueriedResult(queriedVideo: any, video: VideoEntity) {
  assert.equal(queriedVideo.title, video.title, 'Should be equal')
  assert.equal(queriedVideo.description, video.description, 'Should be equal')
  assert.equal(queriedVideo.duration, video.duration, 'Should be equal')
  assert.equal(queriedVideo.thumbnailUrl, video.thumbnailUrl, 'Should be equal')
  assert.equal(queriedVideo.isExplicit, video.isExplicit, 'Should be equal')
  assert.equal(queriedVideo.isPublic, video.isPublic, 'Should be equal')
}

export default async function createVideo({ api, query }: FlowProps): Promise<void> {
  const debug = extendDebug('flow:creatingVideo')
  debug('Started')

  const channelTitle = 'New channel example'
  const createVideoHappyCaseFixture = createVideoReferencingChannelFixture(api, channelTitle)

  await new FixtureRunner(createVideoHappyCaseFixture).run()

  // Temporary solution (wait 2 minutes)
  await Utils.wait(120000)

  // Perform number of full text searches on Channel title, that is a slight variation on title that one expects would return the video.
  let channelFullTextSearchResult = await query.performFullTextSearchOnChannelTitle('joyjoy')

  assert.equal(channelFullTextSearchResult.data.search.length, 1, 'Should contain exactly one entry')

  // Both channel and video title starts with `Example`
  channelFullTextSearchResult = await query.performFullTextSearchOnChannelTitle('Example')

  assert.equal(channelFullTextSearchResult.data.search.length, 2, 'Should contain two entries')

  // Perform number full text searches on Channel title, that absolutely should NOT return the video.
  channelFullTextSearchResult = await query.performFullTextSearchOnChannelTitle('DeadBeef')

  assert.equal(channelFullTextSearchResult.data.search.length, 0, 'Should be empty')

  channelFullTextSearchResult = await query.performFullTextSearchOnChannelTitle('joy')

  assert.equal(channelFullTextSearchResult.data.search.length, 0, 'Should be empty')

  // Ensure channel contains only one video with right data
  const channelResult = await query.getChannelbyHandle(channelTitle)

  assert.equal(channelResult.data.channels[0].videos.length, 1, 'Given channel should contain exactly one video')

  assertVideoMatchQueriedResult(channelResult.data.channels[0].videos[0], createVideoHappyCaseFixture.videoEntity)

  // Perform number of full text searches on Video title, that is a slight variation on title that one expects would return the video.
  let videoFullTextSearchResult = await query.performFullTextSearchOnVideoTitle('example')

  assert.equal(videoFullTextSearchResult.data.search.length, 2, 'Should contain two entries')

  videoFullTextSearchResult = await query.performFullTextSearchOnVideoTitle('Example joyjoy')

  assert.equal(videoFullTextSearchResult.data.search.length, 1, 'Should contain exactly one video')

  // Perform number full text searches on Video title, that absolutely should NOT return the video.
  videoFullTextSearchResult = await query.performFullTextSearchOnVideoTitle('unknown')

  assert.equal(videoFullTextSearchResult.data.search.length, 0, 'Should be empty')

  videoFullTextSearchResult = await query.performFullTextSearchOnVideoTitle('MediaVideo')

  assert.equal(videoFullTextSearchResult.data.search.length, 0, 'Should be empty')

  debug('Done')
}
