-- returning_with_old_new
UPDATE inventory
SET quantity = quantity - 5
WHERE product_id = 456
RETURNING WITH (OLD AS before, NEW AS after)
    before.quantity AS previous_quantity,
    after.quantity AS current_quantity,
    after.quantity - before.quantity AS change;

update t set x = 1 
returning with (old as o, new as n) o.*, n.*;
