create group analysts;

create group extraordinarily_long_group_name_for_business_intelligence_and_reporting with superuser createdb createrole inherit login replication bypassrls connection limit 250 encrypted password 'an extraordinarily long password value used to exercise formatter wrapping' valid until '2042-02-22' in role analysts, developers, administrators admin current_user, session_user;

-- comments in every position
create /* group */ group /* name */ commented_group /* options */ with /* generic option */ superuser /* connection */ connection /* limit */ limit /* number */ 100 /* in */ in /* role */ role /* first role */ foo /* comma */, /* second role */ bar /* admin */ admin /* admin role */ current_user /* semicolon */;
