use std::collections::HashMap;


use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};


pub enum CurrentScreen {
    Main,       // The main screen with the network, account, transactions, and actions tabs.
    Editing,
    Exiting,    // When the user is exiting the CLI.
}

pub enum ActiveTab {
    Accounts,   // Accounts tab
    Transactions, // Transactions tab
    Actions,    // Actions tab (for solana-keygen, config, etc.)
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    None,         // No editing happening
    PublicKey,    // Editing a public key
    Config,       // Editing configuration settings (e.g., setting the cluster)
    KeypairFile,  // Editing the keypair file path
    SolAmount,    // Editing the amount of SOL for transactions
    Network,      // Editing the network to operate
}

pub enum Network {
    Mainnet,
    Devnet,
    Testnet
}


pub enum ActionType {
    ViewBalance,       // Command to view wallet balance
    ViewAccount,       // Command to view account details
    GenerateKeypair,   // Command to generate a keypair using solana-keygen
    SetConfig,         // Command to set a config value
    Airdrop,           // Command to airdrop SOL
    Transfer,          // Command to transfer SOL
}

pub struct App {
    pub command_input: String,          // Input for commands like 'balance', 'transfer', etc.
    pub keypair_file: String,           // Path for keypair file input
    pub sol_amount: f64,                // Holds the amount of SOL for transactions
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub network:Network,

    pub active_tab:ActiveTab,
}

impl App {
    pub fn new() -> App {
        App {
            command_input: String::new(),
            keypair_file: String::new(),
            sol_amount: 0.0,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            network: Network::Devnet,
            active_tab: ActiveTab::Network,
        }
    }

    pub fn get_balance(&self) {
        // Logic to get and display the balance using the RPC client

        self.currently_editing = CurrentlyEditing::PublicKey;
        println!("Fetching wallet balance...");
        // Example: Fetch balance using `self.rpc_client` and display it
    }

    pub fn get_account_details(&self) {
        // Logic to get and display the balance using the RPC client
        
        self.currently_editing = CurrentlyEditing::PublicKey;
        println!("Fetching wallet balance...");
        // Example: Fetch balance using `self.rpc_client` and display it
    }

    pub fn get_config(&self) {
        // Logic to get and display the balance using the RPC client
        println!("Fetching wallet balance...");
        // Example: Fetch balance using `self.rpc_client` and display it
    }

    pub fn transfer_sol(&mut self) {
        // Logic for transferring SOL based on input values
        
        self.currently_editing = CurrentlyEditing::SolAmount;
        println!("Transferring {} SOL to address...", self.sol_amount);
        // Example: Call Solana RPC methods for transferring SOL
    }

    pub fn set_config(&mut self) {
        println!("set the network and config");
    }

    pub fn toggle_tab(&mut self){
        match self.active_tab{
            ActiveTab::Account => {
                self.active_tab = ActiveTab::Transactions;
            }
            ActiveTab::Transactions => {
                self.active_tab = ActiveTab::Actions;
            }
            Active::Actions => {
                self.active_tab = ActiveTab::Account;
            }
        }
    }
}

