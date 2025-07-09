import wallet from "../turbine-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // reference--- https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure 
        const image = "https://gateway.irys.xyz/6ednY51nmz9YCPtkP7fyFEWGPn6miKXerrmNyE4qzoff"
        const metadata = {
            name: "CuteMen",
            symbol: "C",
            description: "Mens are cute",
            image,
            attributes: [
                {trait_type: 'rarity', value: 'Legendary'},
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://gateway.irys.xyz/6ednY51nmz9YCPtkP7fyFEWGPn6miKXerrmNyE4qzoff"
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
