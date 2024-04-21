import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
    TOKEN_PROGRAM_ID,
    MINT_SIZE,
    createAssociatedTokenAccountInstruction,
    getAssociatedTokenAddress,
    createInitializeMintInstruction,
    AccountLayout,
    createApproveInstruction,
    getOrCreateAssociatedTokenAccount,
    approve,
    transfer
} from "@solana/spl-token"
import { assert } from "chai";
import { PeteToken } from "../target/types/pete_token";
import { PeteStaking } from "../target/types/pete_staking";

describe("pete-token", () => {
    anchor.setProvider(anchor.AnchorProvider.env());

    const token_program = anchor.workspace.PeteToken as Program<PeteToken>
    const staking_program = anchor.workspace.PeteStaking as Program<PeteStaking>

    const mintKey = anchor.web3.Keypair.generate()
    const guest = anchor.web3.Keypair.generate()

    const staking_contract_address = new anchor.web3.PublicKey("CTg35G6Cin3iQZHe8i5pN9rJ5ajSyCN2sjvDmVfCyVpi")

    let associated_token_account = undefined

    it("Mint token", async () => {
        const key = anchor.AnchorProvider.env().wallet.publicKey;
        // const key = anchor.web3.Keypair.generate()


        const lamports = await token_program.provider.connection.getMinimumBalanceForRentExemption(
            MINT_SIZE
        )

        associated_token_account = await getAssociatedTokenAddress(mintKey.publicKey, key)

        // Fires a list of instructions
        const mint_tx = new anchor.web3.Transaction().add(
            // Use anchor to create an account from the mint key that we created
            anchor.web3.SystemProgram.createAccount({
                fromPubkey: key,
                newAccountPubkey: mintKey.publicKey,
                space: MINT_SIZE,
                programId: TOKEN_PROGRAM_ID,
                lamports,
            }),
            // Fire a transaction to create our mint account that is controlled by our anchor wallet
            createInitializeMintInstruction(
                mintKey.publicKey, 0, key, key
            ),
            // Create the ATA account that is associated with our mint on our anchor wallet
            createAssociatedTokenAccountInstruction(
                key, associated_token_account, key, mintKey.publicKey
            )
        );

        const res = await anchor.AnchorProvider.env().sendAndConfirm(mint_tx, [mintKey]);

        console.log(
            await token_program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
        );
      
        console.log("Account: ", res);
        console.log("Mint key: ", mintKey.publicKey.toString());
        console.log("User: ", key.toString());

        // Executes our code to mint our token into our specified ATA
        await token_program.methods.mintToken().accounts({
            mint: mintKey.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            tokenAccount: associated_token_account,
            payer: key
        }).rpc();

        // Get minted token amount on the ATA for our anchor wallet
        const minted = (await token_program.provider.connection.getParsedAccountInfo(associated_token_account)).value.data
        
        console.log("Minted:", minted)

        const owner_balance = await getBalance(key, mintKey.publicKey)

        console.log("owner balance:", owner_balance)

        const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
            anchor.AnchorProvider.env().connection,
            anchor.Wallet.local().payer,
            mintKey.publicKey,
            key
        )

        const toTokenAccount = await getOrCreateAssociatedTokenAccount(
            anchor.AnchorProvider.env().connection,
            anchor.Wallet.local().payer,
            mintKey.publicKey,
            guest.publicKey
        )
        
        const stakingTokenAccount = await getOrCreateAssociatedTokenAccount(
            anchor.AnchorProvider.env().connection,
            anchor.Wallet.local().payer,
            mintKey.publicKey,
            staking_contract_address
        )

        const approveInstruction = createApproveInstruction(
            mintKey.publicKey,
            toTokenAccount.address,
            key,
            10
        )

        const approve_tx = new anchor.web3.Transaction().add(approveInstruction)
        // const approve_confirmed = await anchor.AnchorProvider.env().sendAndConfirm(approve_tx, [])

        await token_program.methods.transfer(new anchor.BN(1000)).accounts({
            tokenProgram: TOKEN_PROGRAM_ID,
            from: fromTokenAccount.address,
            signer: key,
            to: stakingTokenAccount.address
        }).rpc()

        await staking_program.methods.deposit(new anchor.BN(1000)).accounts({
            from: fromTokenAccount.address,
            to: stakingTokenAccount.address,
            autority: key,
            tokenProgram: TOKEN_PROGRAM_ID
        }).rpc()

        const owner_balance_after_transfer = await getBalance(key, mintKey.publicKey)
        console.log("owner balance after transfer:", owner_balance_after_transfer)

        const staking_contract_balance = await getBalance(staking_contract_address, mintKey.publicKey)
        console.log("guest balance after transfer:", staking_contract_balance)
    })
})

const getBalance = async (key: anchor.web3.PublicKey, mint: anchor.web3.PublicKey) => {
    const token_account = await anchor.AnchorProvider.env().connection.getTokenAccountsByOwner(
        key,
        {
            mint
        }
    )

    const balance = token_account.value.length > 0 ? AccountLayout.decode(token_account.value[0].account.data).amount : 0

    return balance
}

// describe("pete-staking", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   // const staking_program = anchor.workspace.PeteStaking as Program<PeteStaking>;
//   const token_program = anchor.workspace.PeteToken as Program<PeteToken>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     // const tx = await staking_program.methods.initialize().rpc();
//     // console.log("Your transaction signature", tx);

//     const tx1 = await token_program.methods.mintToken().rpc();
//     console.log("Token transaction", tx1);
//   });
// });
