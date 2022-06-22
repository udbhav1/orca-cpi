import * as assert from "assert";
import { Decimal } from "decimal.js";
import * as splToken from '@solana/spl-token';
import * as web3 from '@solana/web3.js';
import {
  PDAUtil,
  toTx,
  WhirlpoolContext,
  WhirlpoolIx,
} from "@orca-so/whirlpools-sdk";
import { MathUtil, deriveATA, Percentage } from "@orca-so/common-sdk";
import * as anchor from "@project-serum/anchor";

const defaultInitSqrtPrice = MathUtil.toX64_BN(new splToken.u64(1));

export async function mintToByAuthority(
  provider: anchor.Provider,
  mint: web3.PublicKey,
  destination: web3.PublicKey,
  amount: number | anchor.BN
): Promise<string> {
  const tx = new web3.Transaction();
  tx.add(
    splToken.Token.createMintToInstruction(
      splToken.TOKEN_PROGRAM_ID,
      mint,
      destination,
      provider.wallet.publicKey,
      [],
      amount
    )
  );
  return provider.send(tx, [], { commitment: "confirmed" });
}

export async function createAssociatedTokenAccount(
  provider: anchor.Provider,
  mint: web3.PublicKey,
  owner: web3.PublicKey,
  payer: web3.PublicKey
) {
  const ataAddress = await deriveATA(owner, mint);

  const instr = splToken.Token.createAssociatedTokenAccountInstruction(
    splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
    splToken.TOKEN_PROGRAM_ID,
    mint,
    ataAddress,
    owner,
    payer
  );
  const tx = new web3.Transaction();
  tx.add(instr);
  await provider.send(tx, [], { commitment: "confirmed" });
  return ataAddress;
}

export async function createAndMintToAssociatedTokenAccount(
  provider: anchor.Provider,
  mint: web3.PublicKey,
  amount: number | anchor.BN,
  destinationWallet?: web3.PublicKey,
  payer?: web3.PublicKey
): Promise<web3.PublicKey> {
  const destinationWalletKey = destinationWallet ? destinationWallet : provider.wallet.publicKey;
  const payerKey = payer ? payer : provider.wallet.publicKey;
  const tokenAccount = await createAssociatedTokenAccount(
    provider,
    mint,
    destinationWalletKey,
    payerKey
  );
  await mintToByAuthority(provider, mint, tokenAccount, amount);
  return tokenAccount;
}

async function mintTokensToTestAccount(
  provider: anchor.Provider,
  tokenAMint: web3.PublicKey,
  tokenMintForA: number,
  tokenBMint: web3.PublicKey,
  tokenMintForB: number,
  destinationWallet?: web3.PublicKey
) {
  const userTokenAAccount = await createAndMintToAssociatedTokenAccount(
    provider,
    tokenAMint,
    tokenMintForA,
    destinationWallet
  );
  const userTokenBAccount = await createAndMintToAssociatedTokenAccount(
    provider,
    tokenBMint,
    tokenMintForB,
    destinationWallet
  );

  return [userTokenAAccount, userTokenBAccount];
}

const generateDefaultInitFeeTierParams = (
  context: WhirlpoolContext,
  whirlpoolsConfigKey: web3.PublicKey,
  whirlpoolFeeAuthority: web3.PublicKey,
  tickSpacing: number,
  defaultFeeRate: number,
  funder?: web3.PublicKey
) => {
  const feeTierPda = PDAUtil.getFeeTier(
    context.program.programId,
    whirlpoolsConfigKey,
    tickSpacing
  );
  return {
    feeTierPda,
    whirlpoolsConfig: whirlpoolsConfigKey,
    tickSpacing,
    defaultFeeRate,
    feeAuthority: whirlpoolFeeAuthority,
    funder: funder || context.wallet.publicKey,
  };
};

export const generateDefaultConfigParams = (
  context: WhirlpoolContext,
  funder?: anchor.web3.PublicKey
): {
  configInitInfo;
  configKeypairs;
} => {
  const configKeypairs = {
    feeAuthorityKeypair: anchor.web3.Keypair.generate(),
    collectProtocolFeesAuthorityKeypair: anchor.web3.Keypair.generate(),
    rewardEmissionsSuperAuthorityKeypair: anchor.web3.Keypair.generate(),
  };
  const configInitInfo = {
    whirlpoolsConfigKeypair: anchor.web3.Keypair.generate(),
    feeAuthority: configKeypairs.feeAuthorityKeypair.publicKey,
    collectProtocolFeesAuthority: configKeypairs.collectProtocolFeesAuthorityKeypair.publicKey,
    rewardEmissionsSuperAuthority: configKeypairs.rewardEmissionsSuperAuthorityKeypair.publicKey,
    defaultProtocolFeeRate: 300,
    funder: funder || context.wallet.publicKey,
  };
  return { configInitInfo, configKeypairs };
};

/**
 * Initialize a brand new WhirlpoolsConfig account and construct a set of InitPoolParams
 * that can be used to initialize a pool with.
 * @param client - an instance of whirlpool client containing the program & provider
 * @param initSqrtPrice - the initial sqrt-price for this newly generated pool
 * @returns An object containing the params used to init the config account & the param that can be used to init the pool account.
 */
async function buildTestPoolParams(
  ctx: WhirlpoolContext,
  tickSpacing: number,
  defaultFeeRate = 3000,
  initSqrtPrice = defaultInitSqrtPrice,
  funder?: web3.PublicKey
) {
  const { configInitInfo, configKeypairs } = generateDefaultConfigParams(ctx);
  await toTx(ctx, WhirlpoolIx.initializeConfigIx(ctx.program, configInitInfo)).buildAndExecute();

  const { params: feeTierParams } = await initFeeTier(
    ctx,
    configInitInfo,
    configKeypairs.feeAuthorityKeypair,
    tickSpacing,
    defaultFeeRate
  );
  const poolInitInfo = await generateDefaultInitPoolParams(
    ctx,
    configInitInfo.whirlpoolsConfigKeypair.publicKey,
    feeTierParams.feeTierPda.publicKey,
    tickSpacing,
    initSqrtPrice,
    funder
  );
  return {
    configInitInfo,
    configKeypairs,
    poolInitInfo,
    feeTierParams,
  };
}

export async function createMint(
  provider: anchor.Provider,
  authority?: web3.PublicKey
): Promise<web3.PublicKey> {
  if (authority === undefined) {
    authority = provider.wallet.publicKey;
  }
  const mint = web3.Keypair.generate();
  const instructions = await createMintInstructions(provider, authority, mint.publicKey);

  const tx = new web3.Transaction();
  tx.add(...instructions);

  await provider.send(tx, [mint], { commitment: "confirmed" });

  return mint.publicKey;
}

export async function createMintInstructions(
  provider,
  authority: web3.PublicKey,
  mint: web3.PublicKey
) {
  let instructions = [
    web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: mint,
      space: 82,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(82),
      programId: splToken.TOKEN_PROGRAM_ID,
    }),
    splToken.Token.createInitMintInstruction(splToken.TOKEN_PROGRAM_ID, mint, 0, authority, null),
  ];
  return instructions;
}

export const createInOrderMints = async (context: WhirlpoolContext) => {
  const provider = context.provider;
  const tokenXMintPubKey = await createMint(provider);
  const tokenYMintPubKey = await createMint(provider);

  let tokenAMintPubKey, tokenBMintPubKey;
  if (Buffer.compare(tokenXMintPubKey.toBuffer(), tokenYMintPubKey.toBuffer()) < 0) {
    tokenAMintPubKey = tokenXMintPubKey;
    tokenBMintPubKey = tokenYMintPubKey;
  } else {
    tokenAMintPubKey = tokenYMintPubKey;
    tokenBMintPubKey = tokenXMintPubKey;
  }

  return [tokenAMintPubKey, tokenBMintPubKey];
};

const generateDefaultInitPoolParams = async (
  context: WhirlpoolContext,
  configKey: anchor.web3.PublicKey,
  feeTierKey: anchor.web3.PublicKey,
  tickSpacing: number,
  initSqrtPrice = MathUtil.toX64(new Decimal(5)),
  funder?: anchor.web3.PublicKey
) => {
  const [tokenAMintPubKey, tokenBMintPubKey] = await createInOrderMints(context);

  const whirlpoolPda = PDAUtil.getWhirlpool(
    context.program.programId,
    configKey,
    tokenAMintPubKey,
    tokenBMintPubKey,
    tickSpacing
  );
  const tokenVaultAKeypair = web3.Keypair.generate();
  const tokenVaultBKeypair = web3.Keypair.generate();

  return {
    initSqrtPrice,
    whirlpoolsConfig: configKey,
    tokenMintA: tokenAMintPubKey,
    tokenMintB: tokenBMintPubKey,
    whirlpoolPda,
    tokenVaultAKeypair,
    tokenVaultBKeypair,
    feeTierKey,
    tickSpacing,
    funder: funder || context.wallet.publicKey,
  };
};

/**
 * Initialize a brand new set of WhirlpoolsConfig & Whirlpool account
 * @param client - an instance of whirlpool client containing the program & provider
 * @param initSqrtPrice - the initial sqrt-price for this newly generated pool
 * @returns An object containing the params used to initialize both accounts.
 */
async function initTestPool(
  ctx: WhirlpoolContext,
  tickSpacing: number,
  initSqrtPrice = defaultInitSqrtPrice,
  funder?: anchor.web3.Keypair
) {
  const { configInitInfo, poolInitInfo, configKeypairs, feeTierParams } = await buildTestPoolParams(
    ctx,
    tickSpacing,
    3000,
    initSqrtPrice,
    funder?.publicKey
  );

  const tx = toTx(ctx, WhirlpoolIx.initializePoolIx(ctx.program, poolInitInfo));
  if (funder) {
    tx.addSigner(funder);
  }

  return {
    txId: await tx.buildAndExecute(),
    configInitInfo,
    configKeypairs,
    poolInitInfo,
    feeTierParams,
  };
}

async function initFeeTier(
  ctx: WhirlpoolContext,
  configInitInfo,
  feeAuthorityKeypair: anchor.web3.Keypair,
  tickSpacing: number,
  defaultFeeRate: number,
  funder?: anchor.web3.Keypair
) {
  const params = generateDefaultInitFeeTierParams(
    ctx,
    configInitInfo.whirlpoolsConfigKeypair.publicKey,
    configInitInfo.feeAuthority,
    tickSpacing,
    defaultFeeRate,
    funder?.publicKey
  );

  const tx = toTx(ctx, WhirlpoolIx.initializeFeeTierIx(ctx.program, params)).addSigner(
    feeAuthorityKeypair
  );
  if (funder) {
    tx.addSigner(funder);
  }

  return {
    txId: await tx.buildAndExecute(),
    params,
  };
}

module.exports = {
    initTestPool,
    initFeeTier,
    createMint,
    createMintInstructions,
    createInOrderMints,
    generateDefaultInitPoolParams,
    generateDefaultInitFeeTierParams,
    buildTestPoolParams,
    createAndMintToAssociatedTokenAccount,
    createAssociatedTokenAccount,
    mintTokensToTestAccount,
    mintToByAuthority,
    defaultInitSqrtPrice    
}