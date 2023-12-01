#![allow(dead_code)]

use anchor_lang::prelude::*;

use super::client::*;
use super::solana::SolanaCookie;
use super::{send_tx, MintCookie, TestKeypair, UserCookie};
use spl_token_2022::extension::transfer_fee::instruction::initialize_transfer_fee_config;

#[derive(Clone)]
pub struct Token {
    pub index: u16,
    pub mint: MintCookie,
    pub oracle: Pubkey,
    pub mint_info: Pubkey,
}

impl Token {
    pub async fn create(
        mints: Vec<MintCookie>,
        solana: &SolanaCookie,
        owner: TestKeypair,
        payer: TestKeypair,
        is_v2: bool,
    ) -> Vec<Token> {
        let mut tokens = vec![];

        for (index, mint) in mints.iter().enumerate() {

            // Initializing TransferFeeConfig
            if is_v2 {
                let transfer_config_ix = initialize_transfer_fee_config(
                    &spl_token_2022::id(),
                    &mint.pubkey,
                    None,
                    None,
                    100,
                    600000,
                ).unwrap();
    
                let _ = solana.process_transaction(&[transfer_config_ix], Some(&[owner, payer, mint.keypair])).await.unwrap();
            } else {
            }

            let create_stub_oracle_accounts = send_tx(
                solana,
                StubOracleCreate {
                    mint: mint.pubkey,
                    owner,
                    payer,
                },
            )
            .await
            .unwrap();
            let oracle = create_stub_oracle_accounts.oracle;
            send_tx(
                solana,
                StubOracleSetInstruction {
                    owner,
                    mint: mint.pubkey,
                    price: 1.0,
                },
            )
            .await
            .unwrap();
            let token_index = index as u16;
            tokens.push(Token {
                index: token_index,
                mint: *mint,
                oracle,
                mint_info: mint.pubkey,
            });
        }
        tokens
    }
}

pub async fn create_open_orders_indexer(
    solana: &SolanaCookie,
    payer: &UserCookie,
    owner: TestKeypair,
    market: Pubkey,
) -> Pubkey {
    send_tx(
        solana,
        CreateOpenOrdersIndexerInstruction {
            market,
            owner,
            payer: payer.key,
        },
    )
    .await
    .unwrap()
    .open_orders_indexer
}

pub async fn create_open_orders_account(
    solana: &SolanaCookie,
    owner: TestKeypair,
    market: Pubkey,
    account_num: u32,
    payer: &UserCookie,
    delegate: Option<Pubkey>,
) -> Pubkey {
    send_tx(
        solana,
        CreateOpenOrdersAccountInstruction {
            account_num,
            market,
            owner,
            payer: payer.key,
            delegate,
        },
    )
    .await
    .unwrap()
    .open_orders_account
}
