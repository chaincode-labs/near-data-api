UPDATE assets__fungible_tokens
SET burn_amount=$2
WHERE token_id=$1;