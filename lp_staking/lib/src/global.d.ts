import { ActorId } from 'sails-js';

declare global {;
  export type LpStakingError = "errorNotAdmin" | "errorCoinNotPublished" | "errorInvalidLpToken" | "errorLpTokenExist" | "errorWithdrawInsufficient" | "errorInvalidMoveRate" | "errorPidNotExist" | "errorCoinNotRegistered" | "errorMoveRewardOverflow" | "errorInvalidCoinDecimal" | "errorPoolUserInfoNotExist" | "errorZeroAccount" | "errorUpkeepElapsedOverCap" | "errorInputBalance" | "ePoolStillLive" | "eConnectToken" | "eTransferTokenFailed" | "transferTokenFromFailed" | "transferTokenFailed" | "transferFromLiquidityFailed" | "eAmountWithdrawToHight" | "transferLiquidityFailed" | "ePoolEnd";

  export interface PoolStakingInfo {
    total_user: number | string | bigint;
    total_amount: number | string | bigint;
    acc_x_per_share: number | string | bigint;
    x_per_second: number | string | bigint;
    last_reward_timestamp: number | string | bigint;
    end_timestamp: number | string | bigint;
    staked_token: ActorId;
    reward_token: ActorId;
    admin: ActorId;
    precision_factor: number | string | bigint;
  }

  export interface UserInfo {
    amount: number | string | bigint;
    reward_debt: number | string | bigint;
  }

};