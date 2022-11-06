import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';
import { AgoraMarketplace } from '../target/types/agora_marketplace';


describe('agora-marketplace', () => {
    //find the provider and set the anchor provider
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    //get the current program and provider from the IDL
    const agoraProgram = anchor.workspace.AgoraMarketplace as Program<AgoraMarketplace>;
    const agoraProvider = agoraProgram.provider as anchor.AnchorProvider;

    //test inputs
    const username: string = "varun";
    
    it('initialize!', async () => {
        //signer is just the wallet
        const signer = agoraProvider.wallet;

        //finds both PDAs necessary
        const [userPDA, ] = await PublicKey
            .findProgramAddress(
                [
                    anchor.utils.bytes.utf8.encode("username"),
                    anchor.utils.bytes.utf8.encode(username),
                ],
                agoraProgram.programId
            );

        const [profilePDA, ] = await PublicKey
            .findProgramAddress(
                [
                    anchor.utils.bytes.utf8.encode("profile"),
                    provider.wallet.publicKey.toBuffer(),
                ],
                agoraProgram.programId,
            );

        //calls the initialize method
        await agoraProgram.methods
            .initialize(
                username
            )
            .accounts({
                username: userPDA,
                profile: profilePDA,
                user: signer.publicKey,
            })
            .rpc();

        //check that things were updated properly
        let userState = await agoraProgram.account.user.fetch(userPDA);
        //expect(userState.user.toBytes()).to.equal(signer.publicKey.toBytes());

        console.log("stored key: ", userState.user);
        console.log("actual key: ", signer.publicKey);

        let profileState = await agoraProgram.account.profile.fetch(profilePDA);
        
        console.log("timestamp: ", profileState.timestamp);
        console.log("reputation: ", profileState.reputation);
        console.log("sell: ", profileState.sellCount);
        console.log("buy: ", profileState.buyCount);
        console.log("username: ", profileState.username);
    });
});
