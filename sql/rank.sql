select emitted_by_contract_account_id as token_id, count(*) transaction_count from assets__fungible_token_events
where to_timestamp((emitted_at_block_timestamp/1000000000)::int) between current_timestamp - interval '1 day' and current_timestamp
group by emitted_by_contract_account_id
order by transaction_count DESC
limit 10