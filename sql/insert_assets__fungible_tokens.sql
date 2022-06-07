INSERT INTO assets__fungible_tokens(token_id, total_supply, burn_amount, metadata, holder_count)
VALUES ($1, $2, $3, $4, $5)
RETURNING $table_fields;