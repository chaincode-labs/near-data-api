select count(distinct token_new_owner_account_id)::Int as holder_count, emitted_by_contract_account_id as token_id
from assets__fungible_token_events
group by emitted_by_contract_account_id