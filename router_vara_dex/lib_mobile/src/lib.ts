import { GearApi, decodeAddress } from '@gear-js/api';
import { TypeRegistry } from '@polkadot/types';
import { TransactionBuilder, ActorId, getServiceNamePrefix, getFnNamePrefix, ZERO_ADDRESS } from 'sails-js';

export class Program {
  public readonly registry: TypeRegistry;
  public readonly routerService: RouterService;

  constructor(public api: GearApi, public programId?: `0x${string}`) {
    const types: Record<string, any> = {
      RouterError: {"_enum":["PairAlreadyExists","TransferLiquidityFailed","TransferFromLiquidityFailed","TransferFromFailed","InsufficientFee","BurnLiquidityFailed","InsufficientVaraAmount","InsufficientTokenAmount","CreatePairFailed","WithdrawWvaraFailed","DepositWVARAFailed","SwapFailed","MintLiquidityFailed","Expired","PairNotFound","IdenticalAddresses","ZeroAddress","InsufficientBAmount","InsufficientAAmount","InsufficientLiquidity","InvalidPath","InsufficientOutputAmount","InsufficientInputAmount","InvalidLiquidityAmount","ExcessiveInputAmount","TransferFailed"]},
    }

    this.registry = new TypeRegistry();
    this.registry.setKnownTypes({ types });
    this.registry.register(types);

    this.routerService = new RouterService(this);
  }

  newCtorFromCode(code: Uint8Array | Buffer, factory: ActorId, wvara: ActorId, admin_addr: ActorId, fund_addr: ActorId, swap_fee_bps: number | string | bigint): TransactionBuilder<null> {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'upload_program',
      ['New', factory, wvara, admin_addr, fund_addr, swap_fee_bps],
      '(String, [u8;32], [u8;32], [u8;32], [u8;32], u128)',
      'String',
      code,
    );

    this.programId = builder.programId;
    return builder;
  }

  newCtorFromCodeId(codeId: `0x${string}`, factory: ActorId, wvara: ActorId, admin_addr: ActorId, fund_addr: ActorId, swap_fee_bps: number | string | bigint) {
    const builder = new TransactionBuilder<null>(
      this.api,
      this.registry,
      'create_program',
      ['New', factory, wvara, admin_addr, fund_addr, swap_fee_bps],
      '(String, [u8;32], [u8;32], [u8;32], [u8;32], u128)',
      'String',
      codeId,
    );

    this.programId = builder.programId;
    return builder;
  }
}

export class RouterService {
  constructor(private _program: Program) {}

  public addLiquidity(token_a: ActorId, token_b: ActorId, amount_a_desired: number | string | bigint, amount_b_desired: number | string | bigint, amount_a_min: number | string | bigint, amount_b_min: number | string | bigint, to: ActorId, deadline: number | string | bigint): TransactionBuilder<{ ok: [number | string | bigint, number | string | bigint, number | string | bigint] } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: [number | string | bigint, number | string | bigint, number | string | bigint] } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'AddLiquidity', token_a, token_b, amount_a_desired, amount_b_desired, amount_a_min, amount_b_min, to, deadline],
      '(String, String, [u8;32], [u8;32], U256, U256, U256, U256, [u8;32], u64)',
      'Result<(U256, U256, U256), RouterError>',
      this._program.programId
    );
  }

  public addLiquidityVara(token: ActorId, amount_token_desired: number | string | bigint, amount_token_min: number | string | bigint, amount_vara_min: number | string | bigint, to: ActorId, deadline: number | string | bigint): TransactionBuilder<{ ok: [number | string | bigint, number | string | bigint, number | string | bigint] } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: [number | string | bigint, number | string | bigint, number | string | bigint] } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'AddLiquidityVara', token, amount_token_desired, amount_token_min, amount_vara_min, to, deadline],
      '(String, String, [u8;32], U256, U256, U256, [u8;32], u64)',
      'Result<(U256, U256, U256), RouterError>',
      this._program.programId
    );
  }

  public createPair(token_a: ActorId, token_b: ActorId): TransactionBuilder<{ ok: null } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: null } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'CreatePair', token_a, token_b],
      '(String, String, [u8;32], [u8;32])',
      'Result<Null, RouterError>',
      this._program.programId
    );
  }

  public refundToken(token_addr: ActorId, amount: number | string | bigint): TransactionBuilder<{ ok: boolean } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'RefundToken', token_addr, amount],
      '(String, String, [u8;32], U256)',
      'Result<bool, RouterError>',
      this._program.programId
    );
  }

  public refundVara(amount: number | string | bigint): TransactionBuilder<{ ok: boolean } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'RefundVara', amount],
      '(String, String, u128)',
      'Result<bool, RouterError>',
      this._program.programId
    );
  }

  public removeLiquidity(token_a: ActorId, token_b: ActorId, liquidity: number | string | bigint, amount_a_min: number | string | bigint, amount_b_min: number | string | bigint, to: ActorId, deadline: number | string | bigint): TransactionBuilder<{ ok: [number | string | bigint, number | string | bigint] } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: [number | string | bigint, number | string | bigint] } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'RemoveLiquidity', token_a, token_b, liquidity, amount_a_min, amount_b_min, to, deadline],
      '(String, String, [u8;32], [u8;32], U256, U256, U256, [u8;32], u64)',
      'Result<(U256, U256), RouterError>',
      this._program.programId
    );
  }

  public removeLiquidityVara(token: ActorId, liquidity: number | string | bigint, amount_token_min: number | string | bigint, amount_vara_min: number | string | bigint, to: ActorId, deadline: number | string | bigint): TransactionBuilder<{ ok: [number | string | bigint, number | string | bigint] } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: [number | string | bigint, number | string | bigint] } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'RemoveLiquidityVara', token, liquidity, amount_token_min, amount_vara_min, to, deadline],
      '(String, String, [u8;32], U256, U256, U256, [u8;32], u64)',
      'Result<(U256, U256), RouterError>',
      this._program.programId
    );
  }

  public swapExactTokensForTokens(amount_in: number | string | bigint, amount_out_min: number | string | bigint, path: Array<ActorId>, to: ActorId, deadline: number | string | bigint): TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'SwapExactTokensForTokens', amount_in, amount_out_min, path, to, deadline],
      '(String, String, U256, U256, Vec<[u8;32]>, [u8;32], u64)',
      'Result<Vec<U256>, RouterError>',
      this._program.programId
    );
  }

  public swapExactTokensForVara(amount_in: number | string | bigint, amount_out_min: number | string | bigint, path: Array<ActorId>, to: ActorId, deadline: number | string | bigint): TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'SwapExactTokensForVara', amount_in, amount_out_min, path, to, deadline],
      '(String, String, U256, U256, Vec<[u8;32]>, [u8;32], u64)',
      'Result<Vec<U256>, RouterError>',
      this._program.programId
    );
  }

  public swapExactVaraForTokens(amount_out_min: number | string | bigint, path: Array<ActorId>, to: ActorId, deadline: number | string | bigint): TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'SwapExactVaraForTokens', amount_out_min, path, to, deadline],
      '(String, String, U256, Vec<[u8;32]>, [u8;32], u64)',
      'Result<Vec<U256>, RouterError>',
      this._program.programId
    );
  }

  public swapTokensForExactTokens(amount_out: number | string | bigint, amount_in_max: number | string | bigint, path: Array<ActorId>, to: ActorId, deadline: number | string | bigint): TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'SwapTokensForExactTokens', amount_out, amount_in_max, path, to, deadline],
      '(String, String, U256, U256, Vec<[u8;32]>, [u8;32], u64)',
      'Result<Vec<U256>, RouterError>',
      this._program.programId
    );
  }

  public swapTokensForExactVara(amount_out: number | string | bigint, amount_in_max: number | string | bigint, path: Array<ActorId>, to: ActorId, deadline: number | string | bigint): TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'SwapTokensForExactVara', amount_out, amount_in_max, path, to, deadline],
      '(String, String, U256, U256, Vec<[u8;32]>, [u8;32], u64)',
      'Result<Vec<U256>, RouterError>',
      this._program.programId
    );
  }

  public swapVaraForExactTokens(amount_out: number | string | bigint, path: Array<ActorId>, to: ActorId, deadline: number | string | bigint): TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: Array<number | string | bigint> } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'SwapVaraForExactTokens', amount_out, path, to, deadline],
      '(String, String, U256, Vec<[u8;32]>, [u8;32], u64)',
      'Result<Vec<U256>, RouterError>',
      this._program.programId
    );
  }

  public updateFundAddr(new_fund_addr: ActorId): TransactionBuilder<{ ok: boolean } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'UpdateFundAddr', new_fund_addr],
      '(String, String, [u8;32])',
      'Result<bool, RouterError>',
      this._program.programId
    );
  }

  public updateNewAdmin(new_addr: ActorId): TransactionBuilder<{ ok: boolean } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'UpdateNewAdmin', new_addr],
      '(String, String, [u8;32])',
      'Result<bool, RouterError>',
      this._program.programId
    );
  }

  public updateNewFactorty(new_factory_addr: ActorId): TransactionBuilder<{ ok: boolean } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'UpdateNewFactorty', new_factory_addr],
      '(String, String, [u8;32])',
      'Result<bool, RouterError>',
      this._program.programId
    );
  }

  public updateNewWrapvara(new_wvara_addr: ActorId): TransactionBuilder<{ ok: boolean } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'UpdateNewWrapvara', new_wvara_addr],
      '(String, String, [u8;32])',
      'Result<bool, RouterError>',
      this._program.programId
    );
  }

  public updateSwapFeeBps(new_swap_fee_bps: number | string | bigint): TransactionBuilder<{ ok: boolean } | { err: RouterError }> {
    if (!this._program.programId) throw new Error('Program ID is not set');
    return new TransactionBuilder<{ ok: boolean } | { err: RouterError }>(
      this._program.api,
      this._program.registry,
      'send_message',
      ['RouterService', 'UpdateSwapFeeBps', new_swap_fee_bps],
      '(String, String, u128)',
      'Result<bool, RouterError>',
      this._program.programId
    );
  }

  public async getAmountIn(amount_out: number | string | bigint, reserve_in: number | string | bigint, reserve_out: number | string | bigint, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<{ ok: number | string | bigint } | { err: RouterError }> {
    const payload = this._program.registry.createType('(String, String, U256, U256, U256)', ['RouterService', 'GetAmountIn', amount_out, reserve_in, reserve_out]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Result<U256, RouterError>)', reply.payload);
    return result[2].toJSON() as unknown as { ok: number | string | bigint } | { err: RouterError };
  }

  public async getAmountOut(amount_in: number | string | bigint, reserve_in: number | string | bigint, reserve_out: number | string | bigint, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<{ ok: number | string | bigint } | { err: RouterError }> {
    const payload = this._program.registry.createType('(String, String, U256, U256, U256)', ['RouterService', 'GetAmountOut', amount_in, reserve_in, reserve_out]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Result<U256, RouterError>)', reply.payload);
    return result[2].toJSON() as unknown as { ok: number | string | bigint } | { err: RouterError };
  }

  public async getAmountsIn(amount_out: number | string | bigint, path: Array<ActorId>, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<{ ok: Array<number | string | bigint> } | { err: RouterError }> {
    const payload = this._program.registry.createType('(String, String, U256, Vec<[u8;32]>)', ['RouterService', 'GetAmountsIn', amount_out, path]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Result<Vec<U256>, RouterError>)', reply.payload);
    return result[2].toJSON() as unknown as { ok: Array<number | string | bigint> } | { err: RouterError };
  }

  public async getAmountsOut(amount_in: number | string | bigint, path: Array<ActorId>, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<{ ok: Array<number | string | bigint> } | { err: RouterError }> {
    const payload = this._program.registry.createType('(String, String, U256, Vec<[u8;32]>)', ['RouterService', 'GetAmountsOut', amount_in, path]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Result<Vec<U256>, RouterError>)', reply.payload);
    return result[2].toJSON() as unknown as { ok: Array<number | string | bigint> } | { err: RouterError };
  }

  public async getReserves(token_a: ActorId, token_b: ActorId, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<{ ok: [number | string | bigint, number | string | bigint, ActorId] } | { err: RouterError }> {
    const payload = this._program.registry.createType('(String, String, [u8;32], [u8;32])', ['RouterService', 'GetReserves', token_a, token_b]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Result<(U256, U256, [u8;32]), RouterError>)', reply.payload);
    return result[2].toJSON() as unknown as { ok: [number | string | bigint, number | string | bigint, ActorId] } | { err: RouterError };
  }

  public async pairFor(token_a: ActorId, token_b: ActorId, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<{ ok: ActorId } | { err: RouterError }> {
    const payload = this._program.registry.createType('(String, String, [u8;32], [u8;32])', ['RouterService', 'PairFor', token_a, token_b]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Result<[u8;32], RouterError>)', reply.payload);
    return result[2].toJSON() as unknown as { ok: ActorId } | { err: RouterError };
  }

  public async quote(amount_a: number | string | bigint, reserve_a: number | string | bigint, reserve_b: number | string | bigint, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<{ ok: number | string | bigint } | { err: RouterError }> {
    const payload = this._program.registry.createType('(String, String, U256, U256, U256)', ['RouterService', 'Quote', amount_a, reserve_a, reserve_b]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Result<U256, RouterError>)', reply.payload);
    return result[2].toJSON() as unknown as { ok: number | string | bigint } | { err: RouterError };
  }

  public async sortTokens(token_a: ActorId, token_b: ActorId, originAddress?: string, value?: number | string | bigint, atBlock?: `0x${string}`): Promise<{ ok: [ActorId, ActorId] } | { err: RouterError }> {
    const payload = this._program.registry.createType('(String, String, [u8;32], [u8;32])', ['RouterService', 'SortTokens', token_a, token_b]).toHex();
    const reply = await this._program.api.message.calculateReply({
      destination: this._program.programId,
      origin: originAddress ? decodeAddress(originAddress) : ZERO_ADDRESS,
      payload,
      value: value || 0,
      gasLimit: this._program.api.blockGasLimit.toBigInt(),
      at: atBlock || null,
    });
    if (!reply.code.isSuccess) throw new Error(this._program.registry.createType('String', reply.payload).toString());
    const result = this._program.registry.createType('(String, String, Result<([u8;32], [u8;32]), RouterError>)', reply.payload);
    return result[2].toJSON() as unknown as { ok: [ActorId, ActorId] } | { err: RouterError };
  }

  public subscribeToCreatePairEvent(callback: (data: { token_a: ActorId; token_b: ActorId; pair_address: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'RouterService' && getFnNamePrefix(payload) === 'CreatePair') {
        callback(this._program.registry.createType('(String, String, {"token_a":"[u8;32]","token_b":"[u8;32]","pair_address":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { token_a: ActorId; token_b: ActorId; pair_address: ActorId });
      }
    });
  }

  public subscribeToAddLiquidityEvent(callback: (data: { token_a: ActorId; token_b: ActorId; amount_a: number | string | bigint; amount_b: number | string | bigint; to: ActorId; liquidity: number | string | bigint }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'RouterService' && getFnNamePrefix(payload) === 'AddLiquidity') {
        callback(this._program.registry.createType('(String, String, {"token_a":"[u8;32]","token_b":"[u8;32]","amount_a":"U256","amount_b":"U256","to":"[u8;32]","liquidity":"U256"})', message.payload)[2].toJSON() as unknown as { token_a: ActorId; token_b: ActorId; amount_a: number | string | bigint; amount_b: number | string | bigint; to: ActorId; liquidity: number | string | bigint });
      }
    });
  }

  public subscribeToAddLiquidityVARAEvent(callback: (data: { token_a: ActorId; amount_a: number | string | bigint; amount_vara: number | string | bigint; to: ActorId; liquidity: number | string | bigint }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'RouterService' && getFnNamePrefix(payload) === 'AddLiquidityVARA') {
        callback(this._program.registry.createType('(String, String, {"token_a":"[u8;32]","amount_a":"U256","amount_vara":"U256","to":"[u8;32]","liquidity":"U256"})', message.payload)[2].toJSON() as unknown as { token_a: ActorId; amount_a: number | string | bigint; amount_vara: number | string | bigint; to: ActorId; liquidity: number | string | bigint });
      }
    });
  }

  public subscribeToRemoveLiquidityEvent(callback: (data: { token_a: ActorId; token_b: ActorId; amount_a_received: number | string | bigint; amount_b_received: number | string | bigint; to: ActorId; liquidity: number | string | bigint }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'RouterService' && getFnNamePrefix(payload) === 'RemoveLiquidity') {
        callback(this._program.registry.createType('(String, String, {"token_a":"[u8;32]","token_b":"[u8;32]","amount_a_received":"U256","amount_b_received":"U256","to":"[u8;32]","liquidity":"U256"})', message.payload)[2].toJSON() as unknown as { token_a: ActorId; token_b: ActorId; amount_a_received: number | string | bigint; amount_b_received: number | string | bigint; to: ActorId; liquidity: number | string | bigint });
      }
    });
  }

  public subscribeToRemoveLiquidityVARAEvent(callback: (data: { token_a: ActorId; amount_a_received: number | string | bigint; amount_vara_received: number | string | bigint; to: ActorId; liquidity: number | string | bigint }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'RouterService' && getFnNamePrefix(payload) === 'RemoveLiquidityVARA') {
        callback(this._program.registry.createType('(String, String, {"token_a":"[u8;32]","amount_a_received":"U256","amount_vara_received":"U256","to":"[u8;32]","liquidity":"U256"})', message.payload)[2].toJSON() as unknown as { token_a: ActorId; amount_a_received: number | string | bigint; amount_vara_received: number | string | bigint; to: ActorId; liquidity: number | string | bigint });
      }
    });
  }

  public subscribeToSwapExactTokensForTokensEvent(callback: (data: { amount_in: number | string | bigint; amount_out: number | string | bigint; path: Array<ActorId>; to: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'RouterService' && getFnNamePrefix(payload) === 'SwapExactTokensForTokens') {
        callback(this._program.registry.createType('(String, String, {"amount_in":"U256","amount_out":"U256","path":"Vec<[u8;32]>","to":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { amount_in: number | string | bigint; amount_out: number | string | bigint; path: Array<ActorId>; to: ActorId });
      }
    });
  }

  public subscribeToSwapTokensForExactTokensEvent(callback: (data: { amount_out: number | string | bigint; amount_in: number | string | bigint; path: Array<ActorId>; to: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'RouterService' && getFnNamePrefix(payload) === 'SwapTokensForExactTokens') {
        callback(this._program.registry.createType('(String, String, {"amount_out":"U256","amount_in":"U256","path":"Vec<[u8;32]>","to":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { amount_out: number | string | bigint; amount_in: number | string | bigint; path: Array<ActorId>; to: ActorId });
      }
    });
  }

  public subscribeToSwapExactVARAForTokensEvent(callback: (data: { amount_in: number | string | bigint; amount_out: number | string | bigint; path: Array<ActorId>; to: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'RouterService' && getFnNamePrefix(payload) === 'SwapExactVARAForTokens') {
        callback(this._program.registry.createType('(String, String, {"amount_in":"U256","amount_out":"U256","path":"Vec<[u8;32]>","to":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { amount_in: number | string | bigint; amount_out: number | string | bigint; path: Array<ActorId>; to: ActorId });
      }
    });
  }

  public subscribeToSwapTokensForExactVARAEvent(callback: (data: { amount_out: number | string | bigint; amount_in: number | string | bigint; path: Array<ActorId>; to: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'RouterService' && getFnNamePrefix(payload) === 'SwapTokensForExactVARA') {
        callback(this._program.registry.createType('(String, String, {"amount_out":"U256","amount_in":"U256","path":"Vec<[u8;32]>","to":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { amount_out: number | string | bigint; amount_in: number | string | bigint; path: Array<ActorId>; to: ActorId });
      }
    });
  }

  public subscribeToSwapExactTokensForVARAEvent(callback: (data: { amount_in: number | string | bigint; amount_out: number | string | bigint; path: Array<ActorId>; to: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'RouterService' && getFnNamePrefix(payload) === 'SwapExactTokensForVARA') {
        callback(this._program.registry.createType('(String, String, {"amount_in":"U256","amount_out":"U256","path":"Vec<[u8;32]>","to":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { amount_in: number | string | bigint; amount_out: number | string | bigint; path: Array<ActorId>; to: ActorId });
      }
    });
  }

  public subscribeToSwapVARAForExactTokensEvent(callback: (data: { amount_out: number | string | bigint; amount_in: number | string | bigint; path: Array<ActorId>; to: ActorId }) => void | Promise<void>): Promise<() => void> {
    return this._program.api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data: { message } }) => {;
      if (!message.source.eq(this._program.programId) || !message.destination.eq(ZERO_ADDRESS)) {
        return;
      }

      const payload = message.payload.toHex();
      if (getServiceNamePrefix(payload) === 'RouterService' && getFnNamePrefix(payload) === 'SwapVARAForExactTokens') {
        callback(this._program.registry.createType('(String, String, {"amount_out":"U256","amount_in":"U256","path":"Vec<[u8;32]>","to":"[u8;32]"})', message.payload)[2].toJSON() as unknown as { amount_out: number | string | bigint; amount_in: number | string | bigint; path: Array<ActorId>; to: ActorId });
      }
    });
  }
}