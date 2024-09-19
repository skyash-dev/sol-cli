use std::collections::HashMap;


use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};


pub enum CurrentScreen {
    Main,       // The main screen with the network, account, transactions, and actions tabs.
    CommandLog, // The screen displaying the command log.
    Exiting,    // When the user is exiting the CLI.
}

pub enum ActiveTab {
    Network,    // Network Connect tab
    Accounts,   // Accounts tab
    Transactions, // Transactions tab
    Actions,    // Actions tab (for solana-keygen, config, etc.)
}


pub enum CurrentlyEditing {
    None,         // No editing happening
    PublicKey,    // Editing a public key
    Config,       // Editing configuration settings (e.g., setting the cluster)
    KeypairFile,  // Editing the keypair file path
    SolAmount,    // Editing the amount of SOL for transactions
}


pub enum ActionType {
    ViewBalance,       // Command to view wallet balance
    ViewAccount,       // Command to view account details
    GenerateKeypair,   // Command to generate a keypair using solana-keygen
    SetConfig,         // Command to set a config value
    Airdrop,           // Command to airdrop SOL
    Transfer,          // Command to transfer SOL
}

use std::collections::HashMap;
use solana_client::rpc_client::RpcClient;

pub struct App {
    pub command_input: String,          // Input for commands like 'balance', 'transfer', etc.
    pub keypair_file: String,           // Path for keypair file input
    pub config_value: String,           // Holds values for configurations (like cluster setting)
    pub sol_amount: f64,                // Holds the amount of SOL for transactions
    pub accounts: HashMap<String, f64>, // Stores account addresses and balances
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub rpc_client: RpcClient,
}

impl App {
    pub fn new() -> App {
        App {
            command_input: String::new(),
            keypair_file: String::new(),
            config_value: String::new(),
            sol_amount: 0.0,
            accounts: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            rpc_client: RpcClient::new("https://api.devnet.solana.com"), // Example: devnet URL
        }
    }

    pub fn execute_command(&mut self) {
        // Logic to execute the command based on command_input and other fields
        match self.command_input.as_str() {
            "balance" => self.get_balance(),
            "transfer" => self.transfer_sol(),
            "config" => self.set_config(),
            _ => println!("Unknown command"),
        }
        // Clear the command input after execution
        self.command_input = String::new();
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::PublicKey => self.currently_editing = Some(CurrentlyEditing::KeypairFile),
                CurrentlyEditing::KeypairFile => self.currently_editing = Some(CurrentlyEditing::Config),
                CurrentlyEditing::Config => self.currently_editing = Some(CurrentlyEditing::SolAmount),
                CurrentlyEditing::SolAmount => self.currently_editing = None,
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::PublicKey);
        }
    }

    pub fn set_config(&mut self) {
        // Logic for setting Solana configuration
        println!("Setting config value: {}", self.config_value);
        // Set config value, for example: updating the cluster URL
        self.rpc_client = RpcClient::new(self.config_value.clone());
        self.config_value = String::new();
    }

    pub fn get_balance(&self) {
        // Logic to get and display the balance using the RPC client
        println!("Fetching wallet balance...");
        // Example: Fetch balance using `self.rpc_client` and display it
    }

    pub fn transfer_sol(&mut self) {
        // Logic for transferring SOL based on input values
        println!("Transferring {} SOL to address...", self.sol_amount);
        // Example: Call Solana RPC methods for transferring SOL
        self.sol_amount = 0.0;
    }

    pub fn print_accounts(&self) {
        // Prints out all stored accounts and their balances
        for (account, balance) in &self.accounts {
            println!("Account: {}, Balance: {} SOL", account, balance);
        }
    }
}

