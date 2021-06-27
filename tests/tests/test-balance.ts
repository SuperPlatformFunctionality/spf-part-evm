import { expect } from "chai";
import { Keyring } from "@polkadot/keyring";
import { step } from "mocha-steps";

import { createAndFinalizeBlock, describeWithMoonbeam, customRequest } from "./util";
import { GENESIS_ACCOUNT_BALANCE } from "./constants";
import { Event } from "@polkadot/types/interfaces";

describeWithMoonbeam("Moonbeam RPC (Balance)", `simple-specs.json`, (context) => {
  const GENESIS_ACCOUNT = "0x6be02d1d3665660d22ff9624b7be0551ee1ac91b";
  const GENESIS_ACCOUNT_PRIVATE_KEY =
    "0x99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342";
  const TEST_ACCOUNT = "0x1111111111111111111111111111111111111111";

  step("genesis balance is setup correctly (web3)", async function () {
    expect(await context.web3.eth.getBalance(GENESIS_ACCOUNT)).to.equal(
      GENESIS_ACCOUNT_BALANCE.toString()
    );
  });
  step("genesis balance is setup correctly (polkadotJs)", async function () {
    const account = await context.polkadotApi.query.system.account(GENESIS_ACCOUNT);
    expect(account.data.free.toString()).to.equal(GENESIS_ACCOUNT_BALANCE.toString());
  });

  step("balance to be updated after transfer", async function () {
    this.timeout(15000);

    const tx = await context.web3.eth.accounts.signTransaction(
      {
        from: GENESIS_ACCOUNT,
        to: TEST_ACCOUNT,
        value: "0x200", // Must be higher than ExistentialDeposit (0)
        gasPrice: "0x01",
        gas: "0x100000",
      },
      GENESIS_ACCOUNT_PRIVATE_KEY
    );
    await customRequest(context.web3, "eth_sendRawTransaction", [tx.rawTransaction]);
    await createAndFinalizeBlock(context.polkadotApi);
    expect(await context.web3.eth.getBalance(GENESIS_ACCOUNT)).to.equal(
      (GENESIS_ACCOUNT_BALANCE - 0x200n - 21000n).toString()
    );
    expect(await context.web3.eth.getBalance(TEST_ACCOUNT)).to.equal("512");
  });

  step("read ethereum.transact extrinsic events", async function () {
    const signedBlock = await context.polkadotApi.rpc.chain.getBlock();
    const allRecords = await context.polkadotApi.query.system.events.at(
      signedBlock.block.header.hash
    );

    // map between the extrinsics and events
    signedBlock.block.extrinsics.forEach(({ method: { method, section } }, index) => {
      // filter the specific events based on the phase and then the
      // index of our extrinsic in the block
      const events: Event[] = allRecords
        .filter(({ phase }) => phase.isApplyExtrinsic && phase.asApplyExtrinsic.eq(index))
        .map(({ event }) => event);

      switch (index) {
        // First 3 events:
        // timestamp.set:: system.ExtrinsicSuccess
        // parachainUpgrade.setValidationData:: system.ExtrinsicSuccess
        // authorInherent.setAuthor:: system.ExtrinsicSuccess
        case 0:
        case 1:
        case 2:
          expect(
            events.length === 1 && context.polkadotApi.events.system.ExtrinsicSuccess.is(events[0])
          ).to.be.true;
          break;
        // Fourth event: ethereum.transact:: system.NewAccount, balances.Endowed, (?),
        // ethereum.Executed, system.ExtrinsicSuccess
        case 3:
          expect(section === "ethereum" && method === "transact").to.be.true;
          expect(events.length === 4);
          expect(context.polkadotApi.events.system.NewAccount.is(events[0])).to.be.true;
          expect(context.polkadotApi.events.balances.Endowed.is(events[1])).to.be.true;
          // TODO: what event was inserted here?
          expect(context.polkadotApi.events.ethereum.Executed.is(events[3])).to.be.true;
          expect(context.polkadotApi.events.system.ExtrinsicSuccess.is(events[4])).to.be.true;
          break;
        default:
          throw new Error(`Unexpected extrinsic`);
      }
    });
  });

  step("balance should be the same on polkadot/web3", async function () {
    this.timeout(15000);

    const tx = await context.web3.eth.accounts.signTransaction(
      {
        from: GENESIS_ACCOUNT,
        to: TEST_ACCOUNT,
        value: "0x200", // Must be higher than ExistentialDeposit (currently 0)
        gasPrice: "0x01",
        gas: "0x100000",
      },
      GENESIS_ACCOUNT_PRIVATE_KEY
    );
    await customRequest(context.web3, "eth_sendRawTransaction", [tx.rawTransaction]);
    await createAndFinalizeBlock(context.polkadotApi);
    expect(await context.web3.eth.getBalance(GENESIS_ACCOUNT)).to.equal(
      (await context.polkadotApi.query.system.account(GENESIS_ACCOUNT)).data.free.toString()
    );
  });

  const TEST_ACCOUNT_2 = "0x1111111111111111111111111111111111111112";
  step("transfer from polkadotjs should appear in ethereum", async function () {
    this.timeout(15000);

    const keyring = new Keyring({ type: "ethereum" });
    const testAccount = await keyring.addFromUri(GENESIS_ACCOUNT_PRIVATE_KEY, null, "ethereum");
    await context.polkadotApi.tx.balances.transfer(TEST_ACCOUNT_2, 123).signAndSend(testAccount);

    await createAndFinalizeBlock(context.polkadotApi);
    expect(await context.web3.eth.getBalance(TEST_ACCOUNT_2)).to.equal("123");
  });
});
