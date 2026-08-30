create user app_user;

create user extraordinarily_long_application_reporting_user with login encrypted password 'an-extraordinarily-long-secret-value' valid until '2035-12-31 23:59:59+00' connection limit 25 in role extraordinarily_long_read_only_reporting_role, extraordinarily_long_data_warehouse_role;

-- comments in every position
create /* user */ user /* name */ reporting_user /* with */ with /* login */ login /* encrypted */ encrypted /* password */ password /* secret */ 'secret' /* valid */ valid /* until */ until /* timestamp */ '2030-01-01' /* connection */ connection /* limit */ limit /* count */ 10 /* in */ in /* role option */ role /* first role */ readers /* comma */, /* second role */ writers /* end */;
