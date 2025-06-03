import { Keypair } from "@solana/web3.js";
import fs from "fs";

// read the keypair from the file

const keypair = Keypair.fromSecretKey(
  Uint8Array.from(
    JSON.parse(
      fs.readFileSync(
        "./tests/EquiTE7obovztd2fMPX7HGYhKfYwET79HnUL9fxFLTkx.json",
        "utf8"
      )
    )
  )
);

console.log({
  publicKey: keypair.publicKey.toBase58(),
  secretKey: keypair.secretKey.toString(),
});
