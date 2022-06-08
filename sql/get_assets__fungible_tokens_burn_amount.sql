select
	(case
		when SUM(amount::NUMERIC) is null
		then ''
		else SUM(amount::NUMERIC)::text
	end) as burn_amount,
	emitted_by_contract_account_id as token_id
	from assets__fungible_token_events
where event_kind = 'BURN'
group by emitted_by_contract_account_id