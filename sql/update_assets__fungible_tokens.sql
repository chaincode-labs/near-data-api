UPDATE assets__fungible_tokens
SET token_id=$1, total_supply=$2, burn_amount=$3, metadata=$4, holder_count=$5
RETURNING $table_fields;