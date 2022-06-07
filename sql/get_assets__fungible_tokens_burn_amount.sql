select
	(case
		when SUM(amount::NUMERIC) is null
		then ''
		else SUM(amount::NUMERIC)::text
	end) as burn_amount
	from assets__fungible_token_events
where event_kind = 'BURN' and emitted_by_contract_account_id = $1;
