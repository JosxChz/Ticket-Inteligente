import * as anchor from "@coral-xyz/anchor";
import type { SmartTicket } from "../target/types/smart_ticket";
describe("test-simple", () => {  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SmartTicket as anchor.Program<SmartTicket>;
  

  it("solo prueba conexión", async () => {
    const balance = await program.provider.connection.getBalance(program.provider.publicKey);

    console.log("Balance:", balance);

    if (balance > 0) {
      console.log("TODO BIEN ✅");
    } else {
      throw new Error("Sin SOL ❌");
    }
  });
});