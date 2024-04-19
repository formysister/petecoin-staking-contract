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
    getOrCreateAssociatedTokenAccount
} from "@solana/spl-token"
import { assert } from "chai";
import { PeteToken } from "../target/types/pete_token";

describe("pete-token", () => {
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.PeteToken as Program<PeteToken>

    const mintKey = anchor.web3.Keypair.generate()
    const guest = anchor.web3.Keypair.generate()

    let associated_token_account = undefined

    it("Mint token", async () => {
        const key = anchor.AnchorProvider.env().wallet.publicKey;

        const lamports = await program.provider.connection.getMinimumBalanceForRentExemption(
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
            await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
        );
      
        console.log("Account: ", res);
        console.log("Mint key: ", mintKey.publicKey.toString());
        console.log("User: ", key.toString());

        // Executes our code to mint our token into our specified ATA
        await program.methods.mintToken().accounts({
            mint: mintKey.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            tokenAccount: associated_token_account,
            payer: key
        }).rpc();

        // Get minted token amount on the ATA for our anchor wallet
        const minted = (await program.provider.connection.getParsedAccountInfo(associated_token_account)).value.data
        
        console.log("Minted:", minted)

        const token_account = await anchor.AnchorProvider.env().connection.getTokenAccountsByOwner(
            key,
            {
                mint: mintKey.publicKey
            }
        )


        const balance = token_account.value.length > 0 ? AccountLayout.decode(token_account.value[0].account.data).amount : 0

        console.log("Token balance:", balance)


        const guestATA = await getAssociatedTokenAddress(mintKey.publicKey, guest.publicKey)

        const guestTx = new anchor.web3.Transaction().add(
            createAssociatedTokenAccountInstruction(
                key,
                guestATA,
                guest.publicKey,
                mintKey.publicKey
            )
        )

        await anchor.AnchorProvider.env().sendAndConfirm(guestTx, [])

        const approveInstruction = createApproveInstruction(
            mintKey.publicKey,
            key,
            guestATA,
            100,
            [key]
        )

        const approve_tx = new anchor.web3.Transaction().add(approveInstruction)
        const approve_confirmed = await anchor.AnchorProvider.env().sendAndConfirm(approve_tx)

        console.log(approve_confirmed)
    })
})

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
