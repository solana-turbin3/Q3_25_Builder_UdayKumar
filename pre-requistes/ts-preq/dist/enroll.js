"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const web3_js_1 = require("@solana/web3.js");
const anchor_1 = require("@coral-xyz/anchor");
const Turbin3_prereq_1 = require("./programs/Turbin3_prereq");
const Turbin3_wallet_json_1 = __importDefault(require("./Turbin3-wallet.json"));
const system_1 = require("@coral-xyz/anchor/dist/cjs/native/system");
const MPL_CORE_PROGRAM_ID = new web3_js_1.PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
const PROGRAM_ID = new web3_js_1.PublicKey("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM");
const keypair = web3_js_1.Keypair.fromSecretKey(new Uint8Array(Turbin3_wallet_json_1.default));
const connection = new web3_js_1.Connection("https://api.devnet.solana.com", {
    commitment: "confirmed",
});
//creating anchor provider
const provider = new anchor_1.AnchorProvider(connection, new anchor_1.Wallet(keypair), {
    commitment: "confirmed",
});
// creating our program
const program = new anchor_1.Program(Turbin3_prereq_1.IDL, provider);
//creating PDA
const account_seeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()];
const [account_key, _account_bump] = web3_js_1.PublicKey.findProgramAddressSync(account_seeds, program.programId);
const mintCollection = new web3_js_1.PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
const mintTs = web3_js_1.Keypair.generate();
// (async () => {
//   try {
//     const txhash = await program.methods
//       .initialize("udayempire")
//       .accountsPartial({
//         user: keypair.publicKey,
//         account: account_key,
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([keypair])
//       .rpc();
//     console.log(
//       `Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`
//     );
//   } catch (err) {
//     console.error(`Oops, something went wrong: ${err}`);
//   }
// })();
(() => __awaiter(void 0, void 0, void 0, function* () {
    try {
        const txhash = yield program.methods.submitTs().accountsPartial({
            user: keypair.publicKey,
            account: account_key,
            mint: mintTs.publicKey,
            collection: mintCollection,
            mpl_core_program: MPL_CORE_PROGRAM_ID,
            system_program: system_1.SYSTEM_PROGRAM_ID
        }).signers([keypair, mintTs]).rpc();
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    }
    catch (err) {
        console.error(`Oops! something went wrong: ${err}`);
    }
}))();
