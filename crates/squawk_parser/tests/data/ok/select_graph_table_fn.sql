
-- simple
select * from graph_table(
  foo match - columns (a)
);

-- edge left
select * from graph_table(
  foo match <-[ foo is bar where x > 10 ]-
    where x > 10 and z = 4 
    columns (a)
);
select * from graph_table(
  foo match <- columns (a)
);

-- edge right
select * from graph_table(
  foo match -[ foo is bar where x > 10 ]->
    where x > 10 and z = 4 
    columns (a)
);
select * from graph_table(
  foo match -> columns (a)
);

-- edge any
select * from graph_table(
  foo match -[ foo is bar where x > 10 ]-
    where 1=1
    columns (a)
);
select * from graph_table(
  foo match - columns (a)
);

-- paren graph pattern
select * from graph_table(
  foo match (o is foo where a > 10)
    columns (a)
);

select * from graph_table(
  foo match (((o is foo where a > 10)))
    columns (a)
);

select * from graph_table(
  foo match (<-)
    columns (a)
);

-- graph pattern qualifier
select * from graph_table(
  foo match <- {100} columns (a)
);

select * from graph_table(
  foo match - {1,2} columns (a)
);

select * from graph_table(
  foo match - {,2} columns (a)
);

-- multiple path factors
select * from graph_table(
  foo match -[ foo is bar where x > 10 ]-
    columns (a)
);

select * from graph_table(
  t
  match (c is customers)-[co is customer_orders]->(o is orders where o.ordered_when = date '2024-01-02')
    columns (c.name, c.address)
);

-- complicated

select * from graph_table(
  myshop match
    (a is customers where a.id = 1)
    -[e1 is customer_orders]->
    (b is orders where b.ordered_when > date '2024-01-01')
    -[e2 is order_items]-
    (c is products where c.price > 10)
    <-[e3 is wishlist_items]-
    (d is wishlists)
    columns (a.name, b.order_id, c.name, d.wishlist_name)
);

select * from graph_table(
  myshop match
    (a is customers)-[is customer_orders]->(b is orders),
    (b is orders)-[is order_items]->(c is products),
    (a is customers)-[is customer_wishlists]->(w is wishlists)
    columns (a.name, b.order_id, c.name, w.wishlist_name)
);

select * from graph_table(
  myshop match
    (a is customers where a.address = 'US')
    ->(
      (b is orders where b.ordered_when = date '2024-06-01')
      -[e is order_items]->
      (c is products where c.price > 100)
      where b.order_id > 0
    )
    columns (a.name, c.name)
);

select * from graph_table(
  myshop match
    (a is customers)
    -{1,3}
    (b is orders)
    -[is order_items]-
    (c is products)
    (<-[is wishlist_items]-(d is wishlists))
    ->
    (e is customers where e.name = 'Alice')
    columns (a.name, b.order_id, c.name, d.wishlist_name, e.name)
);

select * from graph_table(
  myshop match
    ((a is customers)
      -[e1 is customer_orders]->
      ((b is orders)
        -[e2 is order_items]->
        (c is products)
        where c.price > 50
      )
      where b.ordered_when > date '2023-01-01'
    )
    columns (a.name, c.name)
);

select * from graph_table(
  myshop match
    (a is customers)
    ->
    (b is orders)
    <-
    (c is wishlists)
    -
    (d is products)
    -[e is order_items]->
    (f is orders)
    <-[g is customer_orders]-
    (h is customers where h.id <> a.id)
    columns (a.name, d.name, h.name)
);
