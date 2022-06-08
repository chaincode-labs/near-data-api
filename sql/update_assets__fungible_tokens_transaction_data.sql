
UPDATE assets__fungible_tokens
SET  transaction_count=$2,transaction_amount=$3
WHERE token_id=$1;