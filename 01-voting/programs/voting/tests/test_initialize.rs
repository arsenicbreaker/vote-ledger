use {
    anchor_lang::{
        solana_program::{
            instruction::Instruction,
            pubkey::Pubkey,
            system_program,
        },
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_initialize() {
    let program_id = voting::id();
    let payer = Keypair::new();

    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/voting.so");

    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let poll_id: u64 = 1;
    let poll_id_bytes = poll_id.to_le_bytes();

    let (poll_account, _bump) = Pubkey::find_program_address(
        &[b"poll".as_ref(), poll_id_bytes.as_ref()],
        &program_id,
    );

    let instruction = Instruction::new_with_bytes(
        program_id,
        &voting::instruction::InitializePoll {
            _poll_id: poll_id,
            start_time: 0,
            end_time: 100,
            name: "Poll pertama".to_string(),
            description: "Deskripsi poll".to_string(),
        }
        .data(),
        voting::accounts::InitializePoll {
            signer: payer.pubkey(),
            poll_account,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();

    let msg = Message::new_with_blockhash(
        &[instruction],
        Some(&payer.pubkey()),
        &blockhash,
    );

    let tx = VersionedTransaction::try_new(
        VersionedMessage::Legacy(msg),
        &[payer],
    )
    .unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok(), "{:?}", res);
}