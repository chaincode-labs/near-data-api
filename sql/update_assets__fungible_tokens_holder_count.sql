UPDATE assets__fungible_tokens
SET holder_count=$2
WHERE token_id=$1;