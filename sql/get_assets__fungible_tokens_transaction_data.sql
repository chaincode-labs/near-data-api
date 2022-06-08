select emitted_by_contract_account_id as token_id, count(*)::Int transaction_count, sum(amount::NUMERIC)::text as transaction_amount from assets__fungible_token_events
where to_timestamp((emitted_at_block_timestamp/1000000000)::int) between current_timestamp - interval '1 day' and current_timestamp and event_kind='TRANSFER'
group by emitted_by_contract_account_id
order by transaction_count DESC;