import ContentDirectoryCommandBase from '../../base/ContentDirectoryCommandBase'
import { flags } from '@oclif/command'
import { JoyBTreeSet } from '@joystream/types/common'
import { DataObjectId } from '@joystream/types/storage'
import { registry } from '@joystream/types'

export default class RemoveChannelAssetsCommand extends ContentDirectoryCommandBase {
  static description = 'Remove data objects associated with the channel or any of its videos.'

  static flags = {
    channelId: flags.integer({
      char: 'c',
      required: true,
      description: 'ID of the Channel',
    }),
    objectId: flags.integer({
      char: 'o',
      required: true,
      multiple: true,
      description: 'ID of the object to remove',
    }),
  }

  async run(): Promise<void> {
    const {
      flags: { channelId, objectId: objectIds },
    } = this.parse(RemoveChannelAssetsCommand)
    // Context
    const account = await this.getRequiredSelectedAccount()
    const channel = await this.getApi().channelById(channelId)
    const actor = await this.getChannelOwnerActor(channel)
    await this.requestAccountDecoding(account)

    this.log('Channel id:', channelId)
    this.log('List of assets to remove:', objectIds)
    await this.requireConfirmation('Do you confirm the provided input?', true)

    await this.sendAndFollowNamedTx(account, 'content', 'removeChannelAssets', [
      actor,
      channelId,
      new (JoyBTreeSet(DataObjectId))(registry, objectIds),
    ])
  }
}
