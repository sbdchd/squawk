-- squawk-ignore-file prefer-bigint-over-int, prefer-robust-stmts
set statement_timeout = '5s';
set lock_timeout = '1s';

--
-- via Postgres' docs
-- 5.15. Property Graphs
--
CREATE TABLE products (
    product_no integer PRIMARY KEY,
    name varchar,
    price numeric
);

CREATE TABLE customers (
    customer_id integer PRIMARY KEY,
    name varchar,
    address varchar
);

CREATE TABLE orders (
    order_id integer PRIMARY KEY,
    ordered_when date
);

CREATE TABLE order_items (
    order_items_id integer PRIMARY KEY,
    order_id integer REFERENCES orders (order_id),
    product_no integer REFERENCES products (product_no),
    quantity integer
);

CREATE TABLE customer_orders (
    customer_orders_id integer PRIMARY KEY,
    customer_id integer REFERENCES customers (customer_id),
    order_id integer REFERENCES orders (order_id)
);

CREATE PROPERTY GRAPH myshop
    VERTEX TABLES (
        products,
        customers,
        orders
    )
    EDGE TABLES (
        order_items SOURCE orders DESTINATION products,
        customer_orders SOURCE customers DESTINATION orders
    );

-- get list of customers active today
SELECT customer_name FROM GRAPH_TABLE (myshop MATCH (c IS customers)-[IS customer_orders]->(o IS orders WHERE o.ordered_when = current_date) COLUMNS (c.name AS customer_name));

-- get list of customers active today
SELECT customers.name FROM customers JOIN customer_orders USING (customer_id) JOIN orders USING (order_id) WHERE orders.ordered_when = current_date;

-- explicit version that doesn't require primary & foreign keys
CREATE PROPERTY GRAPH myshop
    VERTEX TABLES (
        products KEY (product_no),
        customers KEY (customer_id),
        orders KEY (order_id)
    )
    EDGE TABLES (
        order_items KEY (order_items_id)
            SOURCE KEY (order_id) REFERENCES orders (order_id)
            DESTINATION KEY (product_no) REFERENCES products (product_no),
        customer_orders KEY (customer_orders_id)
            SOURCE KEY (customer_id) REFERENCES customers (customer_id)
            DESTINATION KEY (order_id) REFERENCES orders (order_id)
    );

CREATE PROPERTY GRAPH myshop
    VERTEX TABLES (
        products LABEL product,
        customers LABEL customer,
        orders LABEL "order"
    )
    EDGE TABLES (
        order_items SOURCE orders DESTINATION products LABEL contains,
        customer_orders SOURCE customers DESTINATION orders LABEL has_placed
    );

SELECT customer_name FROM GRAPH_TABLE (myshop MATCH (c IS customer)-[IS has_placed]->(o IS "order" WHERE o.ordered_when = current_date) COLUMNS (c.name AS customer_name));


select * from
GRAPH_TABLE (mygraph MATCH (p IS person)-[h IS has]->(a IS account)
             COLUMNS (p.name AS person_name, h.since AS has_account_since, a.num AS account_number));

--
-- via: https://pgql-lang.org
--
create table Persons(
  id bigint primary key,
  name text,
  company_id bigint references Companies
);

create table Companies(
  id bigint primary key,
  name text
);

create table Accounts(
  number bigint primary key,
  account_type text,
  person_id bigint references Persons,
  company_id bigint references Companies
);

create table Transactions(
  id bigint primary key,
  from_account bigint references Accounts,
  to_account bigint references Accounts,
  date timestamptz,
  amount bigint
);

CREATE PROPERTY GRAPH financial_transactions
  VERTEX TABLES (
    -- label that gets used in the query lang below, i.e., IS Person
    Persons LABEL Person PROPERTIES ( name ),
    Companies LABEL Company PROPERTIES ( name ),
    Accounts LABEL Account PROPERTIES ( number )
  )
  EDGE TABLES (
    Transactions
      SOURCE KEY ( from_account ) REFERENCES Accounts ( number )
      DESTINATION KEY ( to_account ) REFERENCES Accounts ( number )
      LABEL transaction
        PROPERTIES ( amount ),
    Accounts AS PersonOwner
      SOURCE KEY ( number ) REFERENCES Accounts ( number )
      DESTINATION Persons
      LABEL owner
        NO PROPERTIES,
    Accounts AS CompanyOwner
      SOURCE KEY ( number ) REFERENCES Accounts ( number )
      DESTINATION Companies
      LABEL owner
        NO PROPERTIES,
    Persons AS worksFor
      SOURCE KEY ( id ) REFERENCES Persons ( id )
      DESTINATION Companies
        NO PROPERTIES
  );

SELECT account_holder, SUM(amount) AS total_transacted_with_Nikita
FROM GRAPH_TABLE (
  financial_transactions
  MATCH (p IS Person) <-[IS owner]- (account1 IS Account),
        (account1) -[t IS transaction]- (account2), /* match both incoming and outgoing transactions */
        (account2 IS Account) -[IS owner]-> (owner IS Person|Company)
  WHERE p.name = 'Nikita'
  COLUMNS (owner.name AS account_holder, t.amount)
)
GROUP BY account_holder
