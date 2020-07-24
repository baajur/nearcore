//! Describes the various costs incurred by creating receipts.
//! We use the following abbreviation for readability:
//! * sir -- sender is receiver. Receipts that are directed by an account to itself are guaranteed
//!   to not be cross-shard which is cheaper than cross-shard. Conversely, when sender is not a
//!   receiver it might or might not be a cross-shard communication.
use num_rational::Rational;
use serde::{Deserialize, Serialize};

pub type Gas = u64;

/// Costs associated with an object that can only be sent over the network (and executed
/// by the receiver).
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Fee {
    /// Fee for sending an object from the sender to itself, guaranteeing that it does not leave
    /// the shard.
    pub send_sir: Gas,
    /// Fee for sending an object potentially across the shards.
    pub send_not_sir: Gas,
    /// Fee for executing the object.
    pub execution: Gas,
}

impl Fee {
    #[inline]
    pub fn send_fee(&self, sir: bool) -> Gas {
        if sir {
            self.send_sir
        } else {
            self.send_not_sir
        }
    }

    pub fn exec_fee(&self) -> Gas {
        self.execution
    }

    /// The minimum fee to send and execute.
    fn min_send_and_exec_fee(&self) -> Gas {
        std::cmp::min(self.send_sir, self.send_not_sir) + self.execution
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct RuntimeFeesConfig {
    /// Describes the cost of creating an action receipt, `ActionReceipt`, excluding the actual cost
    /// of actions.
    pub action_receipt_creation_config: Fee,
    /// Describes the cost of creating a data receipt, `DataReceipt`.
    pub data_receipt_creation_config: DataReceiptCreationConfig,
    /// Describes the cost of creating a certain action, `Action`. Includes all variants.
    pub action_creation_config: ActionCreationConfig,
    /// Describes fees for storage.
    pub storage_usage_config: StorageUsageConfig,

    /// Fraction of the burnt gas to reward to the contract account for execution.
    pub burnt_gas_reward: Rational,

    /// Pessimistic gas price inflation ratio.
    pub pessimistic_gas_price_inflation_ratio: Rational,
}

/// Describes the cost of creating a data receipt, `DataReceipt`.
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct DataReceiptCreationConfig {
    /// Base cost of creating a data receipt.
    pub base_cost: Fee,
    /// Additional cost per byte sent.
    pub cost_per_byte: Fee,
}

/// Describes the cost of creating a specific action, `Action`. Includes all variants.
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct ActionCreationConfig {
    /// Base cost of creating an account.
    pub create_account_cost: Fee,

    /// Base cost of deploying a contract.
    pub deploy_contract_cost: Fee,
    /// Cost per byte of deploying a contract.
    pub deploy_contract_cost_per_byte: Fee,

    /// Base cost of calling a function.
    pub function_call_cost: Fee,
    /// Cost per byte of method name and arguments of calling a function.
    pub function_call_cost_per_byte: Fee,

    /// Base cost of making a transfer.
    pub transfer_cost: Fee,

    /// Base cost of staking.
    pub stake_cost: Fee,

    /// Base cost of adding a key.
    pub add_key_cost: AccessKeyCreationConfig,

    /// Base cost of deleting a key.
    pub delete_key_cost: Fee,

    /// Base cost of deleting an account.
    pub delete_account_cost: Fee,
}

/// Describes the cost of creating an access key.
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct AccessKeyCreationConfig {
    /// Base cost of creating a full access access-key.
    pub full_access_cost: Fee,
    /// Base cost of creating an access-key restricted to specific functions.
    pub function_call_cost: Fee,
    /// Cost per byte of method_names of creating a restricted access-key.
    pub function_call_cost_per_byte: Fee,
}

/// Describes cost of storage per block
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct StorageUsageConfig {
    /// Number of bytes for an account record, including rounding up for account id.
    pub num_bytes_account: u64,
    /// Additional number of bytes for a k/v record
    pub num_extra_bytes_record: u64,
}

impl Default for RuntimeFeesConfig {
    fn default() -> Self {
        #[allow(clippy::unreadable_literal)]
        Self {
            action_receipt_creation_config: Fee {
                send_sir: 151021812500,
                send_not_sir: 151021812500,
                execution: 151021812500,
            },
            data_receipt_creation_config: DataReceiptCreationConfig {
                base_cost: Fee {
                    send_sir: 1068762938750,
                    send_not_sir: 1068762938750,
                    execution: 1068762938750,
                },
                cost_per_byte: Fee {
                    send_sir: 23982274,
                    send_not_sir: 23982274,
                    execution: 23982274,
                },
            },
            action_creation_config: ActionCreationConfig {
                create_account_cost: Fee {
                    send_sir: 130476125000,
                    send_not_sir: 130476125000,
                    execution: 130476125000,
                },
                deploy_contract_cost: Fee {
                    send_sir: 222739562500,
                    send_not_sir: 222739562500,
                    execution: 222739562500,
                },
                deploy_contract_cost_per_byte: Fee {
                    send_sir: 6846508,
                    send_not_sir: 6846508,
                    execution: 6846508,
                },
                function_call_cost: Fee {
                    send_sir: 2614094875000,
                    send_not_sir: 2614094875000,
                    execution: 2614094875000,
                },
                function_call_cost_per_byte: Fee {
                    send_sir: 2521519,
                    send_not_sir: 2521519,
                    execution: 2521519,
                },
                transfer_cost: Fee {
                    send_sir: 159813250000,
                    send_not_sir: 159813250000,
                    execution: 159813250000,
                },
                stake_cost: Fee {
                    send_sir: 167840812500,
                    send_not_sir: 167840812500,
                    execution: 167840812500,
                },
                add_key_cost: AccessKeyCreationConfig {
                    full_access_cost: Fee {
                        send_sir: 137640187500,
                        send_not_sir: 137640187500,
                        execution: 137640187500,
                    },
                    function_call_cost: Fee {
                        send_sir: 135268437500,
                        send_not_sir: 135268437500,
                        execution: 135268437500,
                    },
                    function_call_cost_per_byte: Fee {
                        send_sir: 22361438,
                        send_not_sir: 22361438,
                        execution: 22361438,
                    },
                },
                delete_key_cost: Fee {
                    send_sir: 122405750000,
                    send_not_sir: 122405750000,
                    execution: 122405750000,
                },
                delete_account_cost: Fee {
                    send_sir: 205135750000,
                    send_not_sir: 205135750000,
                    execution: 205135750000,
                },
            },
            storage_usage_config: StorageUsageConfig {
                // See Account in core/primitives/src/account.rs for the data structure.
                // TODO(2291): figure out value for the MainNet.
                num_bytes_account: 100,
                num_extra_bytes_record: 40,
            },
            burnt_gas_reward: Rational::new(3, 10),
            pessimistic_gas_price_inflation_ratio: Rational::new(103, 100),
        }
    }
}

impl RuntimeFeesConfig {
    pub fn free() -> Self {
        let free = Fee { send_sir: 0, send_not_sir: 0, execution: 0 };
        RuntimeFeesConfig {
            action_receipt_creation_config: free.clone(),
            data_receipt_creation_config: DataReceiptCreationConfig {
                base_cost: free.clone(),
                cost_per_byte: free.clone(),
            },
            action_creation_config: ActionCreationConfig {
                create_account_cost: free.clone(),
                deploy_contract_cost: free.clone(),
                deploy_contract_cost_per_byte: free.clone(),
                function_call_cost: free.clone(),
                function_call_cost_per_byte: free.clone(),
                transfer_cost: free.clone(),
                stake_cost: free.clone(),
                add_key_cost: AccessKeyCreationConfig {
                    full_access_cost: free.clone(),
                    function_call_cost: free.clone(),
                    function_call_cost_per_byte: free.clone(),
                },
                delete_key_cost: free.clone(),
                delete_account_cost: free,
            },
            storage_usage_config: StorageUsageConfig {
                num_bytes_account: 0,
                num_extra_bytes_record: 0,
            },
            burnt_gas_reward: Rational::from_integer(0),
            pessimistic_gas_price_inflation_ratio: Rational::from_integer(0),
        }
    }

    /// The minimum amount of gas required to create and execute a new receipt with a function call
    /// action.
    /// This amount is used to determine how many receipts can be created, send and executed for
    /// some amount of prepaid gas using function calls.
    pub fn min_receipt_with_function_call_gas(&self) -> Gas {
        self.action_receipt_creation_config.min_send_and_exec_fee()
            + self.action_creation_config.function_call_cost.min_send_and_exec_fee()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_roundtrip_is_more_expensive() {
        // We have an assumption that the deepest receipts we can create is by creating recursive
        // function call promises (calling function call from a function call).
        // If the cost of a data receipt is cheaper than the cost of a function call, then it's
        // possible to create a promise with a dependency which will be executed in two blocks that
        // is cheaper than just two recursive function calls.
        // That's why we need to enforce that the cost of the data receipt is not less than a
        // function call. Otherwise we'd have to modify the way we compute the maximum depth.
        let transaction_costs = RuntimeFeesConfig::default();
        assert!(
            transaction_costs.data_receipt_creation_config.base_cost.min_send_and_exec_fee()
                >= transaction_costs.min_receipt_with_function_call_gas(),
            "The data receipt cost can't be larger than the cost of a receipt with a function call"
        );
    }
}
