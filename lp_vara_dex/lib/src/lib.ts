import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, ActorId, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

export class Program {
  public readonly registry: TypeRegistry;
  public readonly lpService: LpService;

  constructor(public api: GearApi, public programId?: `0x${string}`) {
    const types: Record<string, any> = {
      LpError: {"_enum":["InsufficientAmount","InsufficientFormerAmount","InsufficientLatterAmount","InsufficientLiquidity","InvalidRecipient","ZeroActorId","TransferFailed","Overflow","DeadlineExceeded","IdenticalTokens","FeeToGettingFailed","InvalidTokens","InvalidRouter","CanNotConnectToVft","InsufficientLiquidityMinted","InsufficientLiquidityBurned","InsufficientOutputAmount","InsufficientInputAmount","KConstant","InvalidTo","CanNotConnectToFactory"]},
    }

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.lpService = new LpService(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer, factory: ActorId, token_a: ActorId, token_b: ActorId, name: string, symbol: string, decimals: number): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', factory, token_a, token_b, name, symbol, decimals],
      '(String, [u8;32], [u8;32], [u8;32], String, String, u8)',
      'String',
      code,
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, factory: ActorId, token_a: ActorId, token_b: ActorId, name: string, symbol: string, decimals: number) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', factory, token_a, token_b, name, symbol, decimals],
      '(String, [u8;32], [u8;32], [u8;32], String, String, u8)',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class LpService {
  constructor(private _program: Program) {}

  public burn(to: ActorId): TransactionBuilder<{ ok: [number | string | bigint, number | string | bigint] } | { err: LpError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: [number | string | bigint, number | string | bigint] } | { err: LpError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpService', 'Burn', to],
      '(String, String, [u8;32])',
      'Result<(U256, U256), LpError>',
      this._program.programId
    );
  }

  public mint(to: ActorId): TransactionBuilder<{ ok: number | string | bigint } | { err: LpError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: number | string | bigint } | { err: LpError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpService', 'Mint', to],
      '(String, String, [u8;32])',
      'Result<U256, LpError>',
      this._program.programId
    );
  }

  public skim(to: ActorId): TransactionBuilder<{ ok: null } | { err: LpError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: null } | { err: LpError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpService', 'Skim', to],
      '(String, String, [u8;32])',
      'Result<Null, LpError>',
      this._program.programId
    );
  }

  public swap(amount0_out: number | string | bigint, amount1_out: number | string | bigint, to: ActorId): TransactionBuilder<{ ok: null } | { err: LpError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: null } | { err: LpError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpService', 'Swap', amount0_out, amount1_out, to],
      '(String, String, U256, U256, [u8;32])',
      'Result<Null, LpError>',
      this._program.programId
    );
  }

  public sync(): TransactionBuilder<{ ok: null } | { err: LpError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: null } | { err: LpError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpService', 'Sync'],
      '(String, String)',
      'Result<Null, LpError>',
      this._program.programId
    );
  }

  public approve(spender: ActorId, value: number | string | bigint): TransactionBuilder<boolean> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<boolean>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpService', 'Approve', spender, value],
      '(String, String, [u8;32], U256)',
      'bool',
      this._program.programId
    );
  }

  public transfer(to: ActorId, value: number | string | bigint): TransactionBuilder<boolean> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<boolean>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpService', 'Transfer', to, value],
      '(String, String, [u8;32], U256)',
      'bool',
      this._program.programId
    );
  }

  public transferFrom(from: ActorId, to: ActorId, value: number | string | bigint): TransactionBuilder<boolean> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<boolean>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['LpService', 'TransferFrom', from, to, value],
      '(String, String, [u8;32], [u8;32], U256)',
      'bool',
      this._program.programId
    );
  }

  public async getReserves(originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<[number | string | bigint, number | string | bigint, number | string | bigint]> {
    const payload = this._program.registry.createType('(String, String)', ['LpService', 'GetReserves']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, (U256, U256, u64))', reply.payload);
    return result[2].toJSON() as unknown as [number | string | bigint, number | string | bigint, number | string | bigint];
  }

  public async allowance(owner: ActorId, spender: ActorId, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<bigint> {
    const payload = this._program.registry.createType('(String, String, [u8;32], [u8;32])', ['LpService', 'Allowance', owner, spender]).toHex();
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

  public async balanceOf(account: ActorId, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<bigint> {
    const payload = this._program.registry.createType('(String, String, [u8;32])', ['LpService', 'BalanceOf', account]).toHex();
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

  public async decimals(originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<number> {
    const payload = this._program.registry.createType('(String, String)', ['LpService', 'Decimals']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, u8)', reply.payload);
    return result[2].toNumber() as unknown as number;
  }

  public async name(originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<string> {
    const payload = this._program.registry.createType('(String, String)', ['LpService', 'Name']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, String)', reply.payload);
    return result[2].toString() as unknown as string;
  }

  public async symbol(originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<string> {
    const payload = this._program.registry.createType('(String, String)', ['LpService', 'Symbol']).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, String)', reply.payload);
    return result[2].toString() as unknown as string;
  }

  public async totalSupply(originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<bigint> {
    const payload = this._program.registry.createType('(String, String)', ['LpService', 'TotalSupply']).toHex();
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

  public subscribeToLPMintEvent(callback: (data: { to: ActorId; amount: number | string | bigint }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpService' && getFnNamePrefix(payload) === 'LPMint') {
        callback(this._program.registry.createType('(String, String, {"to":"[u8;32]","amount":"U256"})', message.payload)[2].toJSON() as unknown as { to: ActorId; amount: number | string | bigint });
      }
    });
  }

  public subscribeToMintEvent(callback: (data: { sender: ActorId; amount: [number | string | bigint, number | string | bigint] }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpService' && getFnNamePrefix(payload) === 'Mint') {
        callback(this._program.registry.createType('(String, String, {"sender":"[u8;32]","amount":"(U256, U256)"})', message.payload)[2].toJSON() as unknown as { sender: ActorId; amount: [number | string | bigint, number | string | bigint] });
      }
    });
  }

  public subscribeToBurnEvent(callback: (data: { sender: ActorId; amount: [number | string | bigint, number | string | bigint]; to: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpService' && getFnNamePrefix(payload) === 'Burn') {
        callback(this._program.registry.createType('(String, String, {"sender":"[u8;32]","amount":"(U256, U256)","to":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { sender: ActorId; amount: [number | string | bigint, number | string | bigint]; to: ActorId });
      }
    });
  }

  /**
   * Should be returned from
   * [`InnerAction::SwapExactTokensForTokens`]/[`InnerAction::SwapTokensForExactTokens`].
  */
  public subscribeToSwapEvent(callback: (data: { sender: ActorId; amount_in: [number | string | bigint, number | string | bigint]; amount_out: [number | string | bigint, number | string | bigint]; to: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpService' && getFnNamePrefix(payload) === 'Swap') {
        callback(this._program.registry.createType('(String, String, {"sender":"[u8;32]","amount_in":"(U256, U256)","amount_out":"(U256, U256)","to":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { sender: ActorId; amount_in: [number | string | bigint, number | string | bigint]; amount_out: [number | string | bigint, number | string | bigint]; to: ActorId });
      }
    });
  }

  public subscribeToGetReservesEvent(callback: (data: { reserve_a: number | string | bigint; reserve_b: number | string | bigint; block_timestamp_last: number | string | bigint }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpService' && getFnNamePrefix(payload) === 'GetReserves') {
        callback(this._program.registry.createType('(String, String, {"reserve_a":"u128","reserve_b":"u128","block_timestamp_last":"u64"})', message.payload)[2].toJSON() as unknown as { reserve_a: number | string | bigint; reserve_b: number | string | bigint; block_timestamp_last: number | string | bigint });
      }
    });
  }

  /**
   * Should be returned from [`InnerAction::Sync`].
  */
  public subscribeToSyncEvent(callback: (data: { reserve_a: number | string | bigint; reserve_b: number | string | bigint }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpService' && getFnNamePrefix(payload) === 'Sync') {
        callback(this._program.registry.createType('(String, String, {"reserve_a":"U256","reserve_b":"U256"})', message.payload)[2].toJSON() as unknown as { reserve_a: number | string | bigint; reserve_b: number | string | bigint });
      }
    });
  }

  /**
   * Should be returned from [`InnerAction::Skim`].
  */
  public subscribeToSkimEvent(callback: (data: { amount_a: number | string | bigint; amount_b: number | string | bigint; to: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpService' && getFnNamePrefix(payload) === 'Skim') {
        callback(this._program.registry.createType('(String, String, {"amount_a":"U256","amount_b":"U256","to":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { amount_a: number | string | bigint; amount_b: number | string | bigint; to: ActorId });
      }
    });
  }

  public subscribeToApprovalEvent(callback: (data: { owner: ActorId; spender: ActorId; value: number | string | bigint }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpService' && getFnNamePrefix(payload) === 'Approval') {
        callback(this._program.registry.createType('(String, String, {"owner":"[u8;32]","spender":"[u8;32]","value":"U256"})', message.payload)[2].toJSON() as unknown as { owner: ActorId; spender: ActorId; value: number | string | bigint });
      }
    });
  }

  public subscribeToTransferEvent(callback: (data: { from: ActorId; to: ActorId; value: number | string | bigint }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'LpService' && getFnNamePrefix(payload) === 'Transfer') {
        callback(this._program.registry.createType('(String, String, {"from":"[u8;32]","to":"[u8;32]","value":"U256"})', message.payload)[2].toJSON() as unknown as { from: ActorId; to: ActorId; value: number | string | bigint });
      }
    });
  }
}