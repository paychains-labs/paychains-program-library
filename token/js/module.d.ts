declare module '@paychains/spl-token' {
  import {Buffer} from 'buffer';
  import {Layout} from '@paychains/buffer-layout';
  import {
    PublicKey,
    Signer,
    TransactionInstruction,
    TransactionSignature,
    Connection,
  } from '@paychains/web3.js';
  import BN from 'bn.js';

  // === client/token.js ===
  export const TOKEN_PROGRAM_ID: PublicKey;
  export const ASSOCIATED_TOKEN_PROGRAM_ID: PublicKey;

  export class u64 extends BN {
    toBuffer(): Buffer;
    static fromBuffer(buffer: Buffer): u64;
  }
  export type AuthorityType =
    | 'MintTokens'
    | 'FreezeAccount'
    | 'AccountOwner'
    | 'CloseAccount';

  export const NATIVE_MINT: PublicKey;
  export const MintLayout: Layout;
  export type MintInfo = {
    mintAuthority: null | PublicKey;
    supply: u64;
    decimals: number;
    isInitialized: boolean;
    freezeAuthority: null | PublicKey;
  };

  export const AccountLayout: Layout;
  export type AccountInfo = {
    address: PublicKey;
    mint: PublicKey;
    owner: PublicKey;
    amount: u64;
    delegate: null | PublicKey;
    delegatedAmount: u64;
    isInitialized: boolean;
    isFrozen: boolean;
    isNative: boolean;
    rentExemptReserve: null | u64;
    closeAuthority: null | PublicKey;
  };
  export type MultisigInfo = {
    m: number;
    n: number;
    initialized: boolean;
    signer1: PublicKey;
    signer2: PublicKey;
    signer3: PublicKey;
    signer4: PublicKey;
    signer5: PublicKey;
    signer6: PublicKey;
    signer7: PublicKey;
    signer8: PublicKey;
    signer9: PublicKey;
    signer10: PublicKey;
    signer11: PublicKey;
  };
  export class Token {
    publicKey: PublicKey;
    programId: PublicKey;
    associatedProgramId: PublicKey;
    payer: Signer;
    constructor(
      connection: Connection,
      publicKey: PublicKey,
      programId: PublicKey,
      payer: Signer,
    );
    static getMinBalanceRentForExemptMint(
      connection: Connection,
    ): Promise<number>;
    static getMinBalanceRentForExemptAccount(
      connection: Connection,
    ): Promise<number>;
    static getMinBalanceRentForExemptMultisig(
      connection: Connection,
    ): Promise<number>;
    static getAssociatedTokenAddress(
      associatedProgramId: PublicKey,
      programId: PublicKey,
      mint: PublicKey,
      owner: PublicKey,
      allowOwnerOffCurve?: boolean,
    ): Promise<PublicKey>;
    static createMint(
      connection: Connection,
      payer: Signer,
      mintAuthority: PublicKey,
      freezeAuthority: PublicKey | null,
      decimals: number,
      programId: PublicKey,
    ): Promise<Token>;
    createAccount(owner: PublicKey): Promise<PublicKey>;
    createAssociatedTokenAccount(owner: PublicKey): Promise<PublicKey>;
    static createWrappedNativeAccount(
      connection: Connection,
      programId: PublicKey,
      owner: PublicKey,
      payer: Signer,
      amount: number,
    ): Promise<PublicKey>;
    createMultisig(m: number, signers: Array<PublicKey>): Promise<PublicKey>;
    getMintInfo(): Promise<MintInfo>;
    getAccountInfo(account: PublicKey): Promise<AccountInfo>;
    getOrCreateAssociatedAccountInfo(owner: PublicKey): Promise<AccountInfo>;
    getMultisigInfo(multisig: PublicKey): Promise<MultisigInfo>;
    transfer(
      source: PublicKey,
      destination: PublicKey,
      owner: Signer | PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
    ): Promise<TransactionSignature>;
    transferChecked(
      source: PublicKey,
      destination: PublicKey,
      owner: any,
      multiSigners: Array<Signer>,
      amount: number | u64,
      decimals: number,
    ): Promise<TransactionSignature>;
    approve(
      account: PublicKey,
      delegate: PublicKey,
      owner: Signer | PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
    ): Promise<void>;
    revoke(
      account: PublicKey,
      owner: Signer | PublicKey,
      multiSigners: Array<Signer>,
    ): Promise<void>;
    setAuthority(
      account: PublicKey,
      newAuthority: PublicKey | null,
      authorityType: AuthorityType,
      currentAuthority: Signer | PublicKey,
      multiSigners: Array<Signer>,
    ): Promise<void>;
    mintTo(
      dest: PublicKey,
      authority: Signer | PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
    ): Promise<void>;
    burn(
      account: PublicKey,
      owner: Signer | PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
    ): Promise<void>;
    freezeAccount(
      account: PublicKey,
      authority: any,
      multiSigners: Array<Signer>,
    ): Promise<void>;
    thawAccount(
      account: PublicKey,
      authority: any,
      multiSigners: Array<Signer>,
    ): Promise<void>;
    closeAccount(
      account: PublicKey,
      dest: PublicKey,
      authority: Signer | PublicKey,
      multiSigners: Array<Signer>,
    ): Promise<void>;
    static createInitMintInstruction(
      programId: PublicKey,
      mint: PublicKey,
      decimals: number,
      mintAuthority: PublicKey,
      freezeAuthority: PublicKey | null,
    ): TransactionInstruction;
    static createInitAccountInstruction(
      programId: PublicKey,
      mint: PublicKey,
      account: PublicKey,
      owner: PublicKey,
    ): TransactionInstruction;
    static createTransferInstruction(
      programId: PublicKey,
      source: PublicKey,
      destination: PublicKey,
      owner: PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
    ): TransactionInstruction;
    static createApproveInstruction(
      programId: PublicKey,
      account: PublicKey,
      delegate: PublicKey,
      owner: PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
    ): TransactionInstruction;
    static createRevokeInstruction(
      programId: PublicKey,
      account: PublicKey,
      owner: PublicKey,
      multiSigners: Array<Signer>,
    ): TransactionInstruction;
    static createSetAuthorityInstruction(
      programId: PublicKey,
      account: PublicKey,
      newAuthority: PublicKey | null,
      authorityType: AuthorityType,
      authority: PublicKey,
      multiSigners: Array<Signer>,
    ): TransactionInstruction;
    static createMintToInstruction(
      programId: PublicKey,
      mint: PublicKey,
      dest: PublicKey,
      authority: PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
    ): TransactionInstruction;
    static createBurnInstruction(
      programId: PublicKey,
      mint: PublicKey,
      account: PublicKey,
      owner: PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
    ): TransactionInstruction;
    static createCloseAccountInstruction(
      programId: PublicKey,
      account: PublicKey,
      dest: PublicKey,
      authority: PublicKey,
      multiSigners: Array<Signer>,
    ): TransactionInstruction;
    static createFreezeAccountInstruction(
      programId: PublicKey,
      account: PublicKey,
      mint: PublicKey,
      authority: PublicKey,
      multiSigners: Array<Signer>,
    ): TransactionInstruction;
    static createThawAccountInstruction(
      programId: PublicKey,
      account: PublicKey,
      mint: PublicKey,
      authority: PublicKey,
      multiSigners: Array<Signer>,
    ): TransactionInstruction;
    static createTransferCheckedInstruction(
      programId: PublicKey,
      source: PublicKey,
      mint: PublicKey,
      destination: PublicKey,
      owner: PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
      decimals: number,
    ): TransactionInstruction;
    static createApproveCheckedInstruction(
      programId: PublicKey,
      account: PublicKey,
      mint: PublicKey,
      delegate: PublicKey,
      owner: PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
      decimals: number,
    ): TransactionInstruction;
    static createMintToCheckedInstruction(
      programId: PublicKey,
      mint: PublicKey,
      dest: PublicKey,
      authority: PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
      decimals: number,
    ): TransactionInstruction;
    static createBurnCheckedInstruction(
      programId: PublicKey,
      mint: PublicKey,
      account: PublicKey,
      owner: PublicKey,
      multiSigners: Array<Signer>,
      amount: number | u64,
      decimals: number,
    ): TransactionInstruction;
    static createSyncNativeInstruction(
      programId: PublicKey,
      nativeAccount: PublicKey,
    ): TransactionInstruction;
    static createAssociatedTokenAccountInstruction(
      associatedProgramId: PublicKey,
      programId: PublicKey,
      mint: PublicKey,
      associatedAccount: PublicKey,
      owner: PublicKey,
      payer: PublicKey,
    ): TransactionInstruction;
  }
}
