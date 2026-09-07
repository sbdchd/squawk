
-- trailing comma and missing comma between value lists
values (1,) (1);

-- extra comma, aka missing tuple
values (1),, (2);

-- trailing row-list comma before a follow token
values (1), order by 1;
