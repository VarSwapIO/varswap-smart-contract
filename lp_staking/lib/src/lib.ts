import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, ActorId, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

export class Program {
  public readonly registry: TypeRegistry;
  public readonly lpStakingService: LpStakingService;

  constructor(public api: GearApi, public programId?: `0x${string}`) {
    const types: Record<string, any> = {
      LpStakingError: {"_enum":["ErrorNotAdmin","ErrorCoinNotPublished","ErrorInvalidLpToken","ErrorLpTokenExist","ErrorWithdrawInsufficient","ErrorInvalidMoveRate","ErrorPidNotExist","ErrorCoinNotRegistered","ErrorMoveRewardOverflow","ErrorInvalidCoinDecimal","ErrorPoolUserInfoNotExist","ErrorZeroAccount","ErrorUpkeepElapsedOverCap","ErrorInputBalance","EPoolStillLive","EConnectToken","ETransferTokenFailed","TransferTokenFromFailed","TransferTokenFailed","TransferFromLiquidityFailed","EAmountWithdrawToHight","TransferLiquidityFailed","EPoolEnd"]},
      PoolStakingInfo: {"total_user":"u64","total_amount":"U256","acc_x_per_share":"U256","x_per_second":"U256","last_reward_timestamp":"u64","end_timestamp":"u64","staked_token":"[u8;32]","reward_token":"[u8;32]","admin":"[u8;32]","precision_factor":"U256"},
      UserInfo: {"amount":"U256","reward_debt":"U256"},
    }

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.lpStakingService = new LpStakingService(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer, end_time: number | string | bigint, staked_token: ActorId, reward_token: ActorId, x_per_second: number | string | bigint, admin: ActorId): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', end_time, staked_token, reward_token, x_per_second, admin],
      '(String, u64, [u8;32], [u8;32], U256, [u8;32])',
      'String',
      code,
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, end_time: number | string | bigint, staked_token: ActorId, reward_token: ActorId, x_per_second: number | string | bigint, admin: ActorId) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', end_time, staked_token, reward_token, x_per_second, admin],
      '(String, u64, [u8;32], [u8;32], U256, [u8;32])',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class LpStakingService {
  constructor(private _program: Program) {}

  public changeRewardToken(new_reward_token: ActorId): TransactionBuilder<{ ok: boolean } | { err: LpStakingError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: LpStakingError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpStakingService', 'ChangeRewardToken', new_reward_token],
      '(String, String, [u8;32])',
      'Result<bool, LpStakingError>',
      this._program.programId
    );
  }

  public deposit(amount: number | string | bigint): TransactionBuilder<{ ok: boolean } | { err: LpStakingError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: LpStakingError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpStakingService', 'Deposit', amount],
      '(String, String, U256)',
      'Result<bool, LpStakingError>',
      this._program.programId
    );
  }

  public recoverToken(token: ActorId): TransactionBuilder<{ ok: boolean } | { err: LpStakingError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: LpStakingError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpStakingService', 'RecoverToken', token],
      '(String, String, [u8;32])',
      'Result<bool, LpStakingError>',
      this._program.programId
    );
  }

  public setAdmin(new_admin: ActorId): TransactionBuilder<{ ok: boolean } | { err: LpStakingError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: LpStakingError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpStakingService', 'SetAdmin', new_admin],
      '(String, String, [u8;32])',
      'Result<bool, LpStakingError>',
      this._program.programId
    );
  }

  public updateEndPool(new_time_end: number | string | bigint): TransactionBuilder<{ ok: boolean } | { err: LpStakingError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: LpStakingError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpStakingService', 'UpdateEndPool', new_time_end],
      '(String, String, u64)',
      'Result<bool, LpStakingError>',
      this._program.programId
    );
  }

  public updateRewardPerSecond(new_reward_per_second: number | string | bigint): TransactionBuilder<{ ok: boolean } | { err: LpStakingError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: LpStakingError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpStakingService', 'UpdateRewardPerSecond', new_reward_per_second],
      '(String, String, U256)',
      'Result<bool, LpStakingError>',
      this._program.programId
    );
  }

  public withdraw(_amount: number | string | bigint): TransactionBuilder<{ ok: boolean } | { err: LpStakingError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: LpStakingError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpStakingService', 'Withdraw', _amount],
      '(String, String, U256)',
      'Result<bool, LpStakingError>',
      this._program.programId
    );
  }

  public async checkLiquidityBalance(_user: ActorId, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<bigint> {
    const payload = this._program.registry.createType('(String, String, [u8;32])', ['LpStakingService', 'CheckLiquidityBalance', _user]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, U256)', reply.payload);
    return result[2].toBigInt() as unknown as bigint;
  }

  public async checkRewardBalance(_user: ActorId, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<bigint> {
    const payload = this._program.registry.createType('(String, String, [u8;32])', ['LpStakingService', 'CheckRewardBalance', _user]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, U256)', reply.payload);
    return result[2].toBigInt() as unknown as bigint;
  }

  public async pendingReward(_user: ActorId, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<bigint> {
    const payload = this._program.registry.createType('(String, String, [u8;32])', ['LpStakingService', 'PendingReward', _user]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, U256)', reply.payload);
    return result[2].toBigInt() as unknown as bigint;
  }

  public async poolInfo(originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<PoolStakingInfo> {
    const payload = this._program.registry.createType('(String, String)', ['LpStakingService', 'PoolInfo']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, PoolStakingInfo)', reply.payload);
    return result[2].toJSON() as unknown as PoolStakingInfo;
  }

  public async userInfo(_user: ActorId, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<UserInfo> {
    const payload = this._program.registry.createType('(String, String, [u8;32])', ['LpStakingService', 'UserInfo', _user]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, UserInfo)', reply.payload);
    return result[2].toJSON() as unknown as UserInfo;
  }

  public subscribeToDepositEvent(callback: (data: { user: ActorId; amount: number | string | bigint; total_lp_staked: number | string | bigint; staked_token: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpStakingService' && getFnNamePrefix(payload) === 'Deposit') {
        callback(this._program.registry.createType('(String, String, {"user":"[u8;32]","amount":"U256","total_lp_staked":"U256","staked_token":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { user: ActorId; amount: number | string | bigint; total_lp_staked: number | string | bigint; staked_token: ActorId });
      }
    });
  }

  public subscribeToWithdrawEvent(callback: (data: { user: ActorId; amount: number | string | bigint; total_lp_staked: number | string | bigint; staked_token: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpStakingService' && getFnNamePrefix(payload) === 'Withdraw') {
        callback(this._program.registry.createType('(String, String, {"user":"[u8;32]","amount":"U256","total_lp_staked":"U256","staked_token":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { user: ActorId; amount: number | string | bigint; total_lp_staked: number | string | bigint; staked_token: ActorId });
      }
    });
  }

  public subscribeToTokenRecoveryEvent(callback: (data: { token: ActorId; amount: number | string | bigint }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpStakingService' && getFnNamePrefix(payload) === 'TokenRecovery') {
        callback(this._program.registry.createType('(String, String, {"token":"[u8;32]","amount":"U256"})', message.payload)[2].toJSON() as unknown as { token: ActorId; amount: number | string | bigint });
      }
    });
  }
}