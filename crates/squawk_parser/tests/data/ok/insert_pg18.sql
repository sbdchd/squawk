-- on conflict do select
INSERT INTO t (a) VALUES (1)
    ON CONFLICT (a) DO SELECT FOR UPDATE WHERE a > 0 RETURNING *;
