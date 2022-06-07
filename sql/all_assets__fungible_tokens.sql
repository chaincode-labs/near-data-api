select action_receipt_actions.args->'args_json'->>'total_supply' as total_supply,action_receipt_actions.args->'args_json'->>'owner_id' as owner_id ,action_receipt_actions.args->'args_json'->>'metadata' as metadata , receipts.receiver_account_id as token_id
from action_receipt_actions
left join receipts on receipts.receipt_id = action_receipt_actions.receipt_id
where action_receipt_actions.action_kind='FUNCTION_CALL' and action_receipt_actions.args->>'method_name'='new'
and action_receipt_actions.args->'args_json'->'metadata'->>'spec'='ft-1.0.0'
limit 10;