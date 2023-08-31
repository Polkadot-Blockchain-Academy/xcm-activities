## Activities

### Test how versioning works

- Use the **mock_version_changer** pallet to be able to control the version that is sent over.
- Fill the appropriate components in the test:
  - Set para version to 1
  - Set relay version to 2
  - Let them talk to each other.
    What events do we expect to see?
  - Upgrade para version to 2.
  - Perform a runtime upgrade.
  - Does the relay get notified?

### Build XCM configurations for requirements

Modify different parts of the [parachain runtime](../xcm-simulator/src/parachain.rs) configuration to accomplish the following goals:

- Remove free execution from relay and add a trader to charge for fee
- Modify your chain to be a 20 byte account instead of a 32 byte account.
  - **Hint: You can use sp_core::H160 as your account 20 type.**
- Change the configuration to accept teleporting instead of reserve transfer assets.
- Add pallet-assets to the parachain and add an asset transactor for it.

Write some tests with the simulator to prove the aforementioned behavior
