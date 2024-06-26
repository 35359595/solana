use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use {
    assert_matches::assert_matches,
    solana_program_test::*,
    solana_sdk::{
        ed25519_instruction::new_ed25519_instruction,
        signature::Signer,
        transaction::{Transaction, TransactionError},
    },
};

fn generate_keypair() -> SigningKey {
    SigningKey::generate(&mut OsRng)
}

#[tokio::test]
async fn test_success() {
    let mut context = ProgramTest::default().start_with_context().await;

    let client = &mut context.banks_client;
    let payer = &context.payer;
    let recent_blockhash = context.last_blockhash;

    let privkey = generate_keypair();
    let message_arr = b"hello";
    let instruction = new_ed25519_instruction(&privkey, message_arr);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    assert_matches!(client.process_transaction(transaction).await, Ok(()));
}

#[tokio::test]
async fn test_failure() {
    let mut context = ProgramTest::default().start_with_context().await;

    let client = &mut context.banks_client;
    let payer = &context.payer;
    let recent_blockhash = context.last_blockhash;

    let privkey = generate_keypair();
    let message_arr = b"hello";
    let mut instruction = new_ed25519_instruction(&privkey, message_arr);

    instruction.data[0] += 1;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );

    assert_matches!(
        client.process_transaction(transaction).await,
        Err(BanksClientError::TransactionError(
            TransactionError::InvalidAccountIndex
        ))
    );
}
