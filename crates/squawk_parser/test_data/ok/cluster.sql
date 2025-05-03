-- simple
cluster;

-- full
cluster (verbose false) foo.bar using idx;

-- options_only
cluster (verbose false);

-- pre_14
cluster verbose foo.bar using idx;

-- pre_17
cluster verbose;

-- pre_8_3
cluster verbose f on foo.bar;

