Status Message
==============

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/near-examples/rust-status-message)

<!-- MAGIC COMMENT: DO NOT DELETE! Everything above this line is hidden on NEAR Examples page -->

This smart contract saves and records the status messages of NEAR accounts that call it.

Windows users: please visit the [Windows-specific README file](README-Windows.md).

## Prerequisites
Ensure `near-cli` is installed by running:

```
near --version
```

If needed, install `near-cli`:

```
npm install near-cli -g
```

Ensure `Rust` is installed by running:

```
rustc --version
```

If needed, install `Rust`:

```
curl https://sh.rustup.rs -sSf | sh
```

Install dependencies

```
npm install
```

Log to console:
```rs
env::log_str(&format!("User Data: {:?}", user_data));
```

## Quick Start
To run this project locally:

1. Prerequisites: Make sure you have Node.js ≥ 12 installed (https://nodejs.org), then use it to install yarn: `npm install --global yarn` (or just `npm i -g yarn`)
2. Run the local development server: `yarn && yarn dev` (see package.json for a full list of scripts you can run with yarn)
Now you'll have a local development environment backed by the NEAR TestNet! Running yarn dev will tell you the URL you can visit in your browser to see the app.

## Building this contract
To make the build process compatible with multiple operating systems, the build process exists as a script in `package.json`.
There are a number of special flags used to compile the smart contract into the wasm file.
Run this command to build and place the wasm file in the `res` directory:
```bash
npm run build
```

**Note**: Instead of `npm`, users of [yarn](https://yarnpkg.com) may run:
```bash
yarn build
```

### Important
If you encounter an error similar to:
>note: the `wasm32-unknown-unknown` target may not be installed

Then run:

```bash
rustup target add wasm32-unknown-unknown
```

## Using this contract

### Web app

Deploy the smart contract to a specific account created with the NEAR Wallet. Then interact with the smart contract using near-api-js on the frontend.

If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

Make sure you have credentials saved locally for the account you want to deploy the contract to. To perform this run the following `near-cli` command:

```
near login
```

Deploy the contract to your NEAR account:

```bash
near deploy --wasmFile res/status_message.wasm --accountId YOUR_ACCOUNT_NAME
```

Build the frontend:

```bash
npm start
```

If all is successful the app should be live at `localhost:1234`!

### Quickest deploy
Build and deploy this smart contract to an development account. This development account will be created automatically and is not intended to be permanent. Please see the "Standard deploy" section for creating a more personalized account to deploy to.

```bash
near dev-deploy --wasmFile res/status_message.wasm --helperUrl https://near-contract-helper.onrender.com
```

Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

>Done deploying to dev-1234567890123

In this instance, the account is `dev-1234567890123`. A file has been created containing the key to the account, located at `neardev/dev-account`. To make the next few steps easier, we're going to set an environment variable containing this development account id and use that when copy/pasting commands.
Run this command to the environment variable:

```bash
source neardev/dev-account.env
```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:
```bash
echo $CONTRACT_NAME

```

The next command will call the contract's `set_status` method:

```bash
near call $CONTRACT_NAME set_status '{"message": "aloha!"}' --accountId $CONTRACT_NAME
```

To retrieve the message from the contract, call `get_status` with the following:

```bash
near view $CONTRACT_NAME get_status '{"account_id": "'$CONTRACT_NAME'"}'
```

### Standard deploy
In this option, the smart contract will get deployed to a specific account created with the NEAR Wallet.

If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

Make sure you have credentials saved locally for the account you want to deploy the contract to. To perform this run the following `near-cli` command:

```
near login
```

Deploy the contract:

```bash
near deploy --wasmFile res/status_message.wasm --accountId YOUR_ACCOUNT_NAME
```

Set a status for your account:

```bash
near call YOUR_ACCOUNT_NAME set_status '{"message": "aloha friend"}' --accountId YOUR_ACCOUNT_NAME
```

Get the status:

```bash
near view YOUR_ACCOUNT_NAME get_status '{"account_id": "YOUR_ACCOUNT_NAME"}'
```

Note that these status messages are stored per account in a `HashMap`. See `src/lib.rs` for the code. We can try the same steps with another account to verify.
**Note**: we're adding `NEW_ACCOUNT_NAME` for the next couple steps.

There are two ways to create a new account:
 - the NEAR Wallet (as we did before)
 - `near create_account NEW_ACCOUNT_NAME --masterAccount YOUR_ACCOUNT_NAME`

Now call the contract on the first account (where it's deployed):

```bash
near call YOUR_ACCOUNT_NAME set_status '{"message": "bonjour"}' --accountId NEW_ACCOUNT_NAME
```

```bash
near view YOUR_ACCOUNT_NAME get_status '{"account_id": "NEW_ACCOUNT_NAME"}'
```

Returns `bonjour`.

Make sure the original status remains:

```bash
near view YOUR_ACCOUNT_NAME get_status '{"account_id": "YOUR_ACCOUNT_NAME"}'
```

## Testing
To test run:
```bash
cargo test --package status-message -- --nocapture
```
# Contract Method
## User
- [User] Get User Information by AccountId
```rs
pub fn get_user(&mut self, user_id: AccountId) -> ProjectUser
```
- [User] Get Project Information by Creator AccountId
```rs
pub fn get_user_projects_created(&mut self, id: AccountId) -> Project
```
- [User] Get list of projects that user funded
```rs
pub fn get_projects_funded(&self) -> Vec<Project>
```
- [User] Get list of projects that user watched
```rs
pub fn get_projects_watched(&self) -> Vec<Project>
```

## Project
- [Project] Get Project Information by ProjectId
```rs
pub fn get_project(&mut self, id: ProjectId) -> Project
```
- [Project] Create Project. Each user can only create one project
```rs
pub fn create_project(&mut self, metadata: String) -> Project
```
- [Project] Add Offer to Project
```rs
pub fn add_project_offer(
        &mut self,
        id: ProjectId,
        price: Balance,
        expires_at: u64,
        metadata: String,
    ) -> Vec<Offer>
```
- [Project] Remove Offer from Project
```rs
pub fn remove_project_offer(&mut self, id: ProjectId, offer_id: String) -> Vec<Offer>
```
- [Project] Update project metadata
```rs
pub fn update_project(&mut self, id: ProjectId, metadata: String) -> Project
```
- [Project] Buy Offer
```rs
pub fn buy_offer(&mut self, project_id: ProjectId, offer_id: String) -> Void
```
- [Project] Approve Project, Release all money to project owner
```rs
pub fn approve_project(&mut self, id: ProjectId, rate: u32, metadata: String) -> Void
```
- [Project] Reject Project, Cashback remain money to pledger
```rs
pub fn reject_project(&mut self, id: ProjectId, rate: u32, metadata: String) -> Void
```
- [Project] Add Project to watchlist
```rs
pub fn add_to_watchlist(&mut self, id: ProjectId) -> Void
```
- [Project] Get all bought offers of a project
```rs
pub fn get_bought_offers(&self, id: ProjectId) -> Vec<BoughtOffer>
```
- [Project] Get list of pledgers of a project
```rs
pub fn get_pledgers(&self, id: ProjectId) -> Vec<ProjectUser>
```
- [Project] Get list of watcher of a project
```rs
pub fn get_watchers(&self, id: ProjectId) -> Vec<ProjectUser>
```