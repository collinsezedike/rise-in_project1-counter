#[cfg(test)]
mod test {
    use crate::{process_instruction, CounterAccount};
    use borsh::BorshDeserialize;
    use solana_program::{account_info::AccountInfo, clock::Epoch, pubkey::Pubkey};
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let accounts = vec![account];

        let mut increment_ixn_data: Vec<u8> = vec![0];
        let mut decrement_ixn_data: Vec<u8> = vec![1];
        let mut update_ixn_data: Vec<u8> = vec![2];
        let reset_ixn_data: Vec<u8> = vec![3];

        // Test increment
        let increment_value = 4u32;
        increment_ixn_data.extend_from_slice(&increment_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &increment_ixn_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            4
        );

        // Test decrement
        let decrement_value = 4u32;
        decrement_ixn_data.extend_from_slice(&decrement_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &decrement_ixn_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        // Test update
        let update_value = 33u32;
        update_ixn_data.extend_from_slice(&update_value.to_le_bytes());
        process_instruction(&program_id, &accounts, &update_ixn_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            33
        );

        // Test reset
        process_instruction(&program_id, &accounts, &reset_ixn_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
