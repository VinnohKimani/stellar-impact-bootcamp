use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal, String};

use crate::{
    error::ContractError,
    events::{Approval, Transfer},
    storage::{AllowanceKey, DataKey},
};

#[contract]
pub struct SibToken;

#[contractimpl]
impl SibToken {
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Allowance(AllowanceKey { from, spender }))
            .unwrap_or(0)
    }

    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) -> Result<(), ContractError> {
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());

        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender: spender.clone(),
        });

        env.storage().persistent().set(&key, &amount);

        Approval {
            from,
            spender,
            amount: amount.try_into().unwrap(),
            live_until_ledger: live_until_ledger.into_val(&env),
        }
        .publish(&env);

        Ok(())
    }

    pub fn transfer(
        env: &Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        from.require_auth();
        let sender_balance = Self::balance(env.clone(), from.clone());
        let receiver_balance = Self::balance(env.clone(), to.clone());

        if sender_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(sender_balance - amount));

        //  Changed storage key from value to correct DataKey enum
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(receiver_balance + amount));

        Transfer {
            from,
            to,
            amount: amount.try_into().unwrap(),
        }
        .publish(env);

        Ok(())
    }

    pub fn decimals(_env: Env) -> u32 {
        18
    }

    pub fn name(env: Env) -> String {
        String::from_str(&env, "SibToken")
    }

    pub fn symbol(env: Env) -> String {
        String::from_str(&env, "SIB")
    }

   /// Transfers tokens from one account to another using an allowance established via approve
    pub fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        // Enforce that the entity initiating this transaction is the authorized spender
        spender.require_auth();

        // Retrieve the current allowance allocation given to this spender by the 'from' account
        let current_allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        // Verify the spender has been allocated enough allowance to handle this transfer volume
        if current_allowance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        // Fetch the active on-chain ledger balances for both the sender and receiver accounts
        let from_balance = Self::balance(env.clone(), from.clone());
        let to_balance = Self::balance(env.clone(), to.clone());

        // Verify the underlying token owner actually has enough tokens to cover the transaction
        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        // Construct the unique state storage key matching this specific (owner, spender) pair
        let allowance_key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender: spender.clone(),
        });
        // Deduct the transferred amount from the spender's remaining allowance in persistent storage
        env.storage()
            .persistent()
            .set(&allowance_key, &(current_allowance - amount));

        // Deduct the token balance from the token owner's account state
        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        // Add the token balance to the destination receiver's account state
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(to_balance + amount));

        // Emit an on-chain Transfer event so off-chain indexers and user interfaces track the movement
        Transfer {
            from,
            to,
            amount: amount.try_into().unwrap(),
        }
        .publish(&env);

        Ok(())
    }

    /// This Destroys a specified amount of tokens from the caller's own balance
    pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), ContractError> {
        // only the true token owner can voluntarily destroy their own tokens
        from.require_auth();

        // Reading the current balance state of the burner
        let from_balance = Self::balance(env.clone(), from.clone());

        // Ensuring that they have enough tokens to fulfill the burn request
        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        //  storage update
        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(from_balance - amount));

        
        Transfer {
            from: from.clone(),
            to: from.clone(), 
            amount: amount.try_into().unwrap(),
        }
        .publish(&env);

        Ok(())
    }

    /// Destroying tokens from a target account using a previously granted allowance allocation
    pub fn burn_from(
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        // Enforce that the transaction signer is the designated spender profile
        spender.require_auth();

        // Retrieve how many tokens this spender is allowed to manage on behalf of the owner
        let current_allowance = Self::allowance(env.clone(), from.clone(), spender.clone());

        // Verifying that  the allowance bounds cover this burn quantity
        if current_allowance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        // Checking the actual token balance of the target account
        let from_balance = Self::balance(env.clone(), from.clone());

        // Confirming that the account physically contains enough tokens to burn
        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        // Reconstructing the allowance mapping key
        let allowance_key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender: spender.clone(),
        });
        // Decreasing the spender's remaining allowance 
        env.storage()
            .persistent()
            .set(&allowance_key, &(current_allowance - amount));

        // Decreasing the token supply balance 
        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(from_balance - amount));


        Transfer {
            from: from.clone(),
            to: from.clone(),
            amount: amount.try_into().unwrap(),
        }
        .publish(&env);

        Ok(())
    }

    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), ContractError> {
        // Reading  the current balance of the target receiver profile (defaults to 0 if new)
        let target_balance = Self::balance(env.clone(), to.clone());

        // Updating storage by saving the old balance increased by the newly minted token allocation
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(target_balance + amount));

        
        Transfer {
            from: to.clone(), 
            to,
            amount: amount.try_into().unwrap(),
        }
        .publish(&env);

        Ok(())
    }
}